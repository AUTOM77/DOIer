use actix_web::{http::header as actix_header, HttpResponse};
use futures_util::TryStreamExt as _;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{header, redirect::Policy, Client, Response};
use std::time::Duration;
use tokio::time::sleep;

use super::doi::Doi;

#[derive(Clone)]
pub struct Paper {
    client: Client,
}

impl Paper {
    pub fn new() -> Self {
        let client = Client::builder()
            .redirect(Policy::limited(12))
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(120))
            .pool_max_idle_per_host(8)
            .cookie_store(true)  // Enable cookie storage for session handling
            .gzip(true)         // Enable gzip decompression
            .brotli(true)       // Enable brotli decompression
            .deflate(true)      // Enable deflate decompression
            .build()
            .expect("reqwest client");
        Self { client }
    }

    pub async fn fetch_pdf_httpresponse(&self, doi: &str) -> actix_web::Result<HttpResponse> {
        let doi_struct = Doi::new(doi);
        let pdf_urls = doi_struct.construct_pdf_urls();

        for pdf_url in &pdf_urls {
            eprintln!("Trying direct URL: {}", pdf_url);

            // IEEE requires visiting document page first to establish session
            if pdf_url.contains("ieeexplore.ieee.org/stampPDF") {
                if let Some(arnumber) = pdf_url.split("arnumber=").nth(1) {
                    let doc_url = format!("https://ieeexplore.ieee.org/document/{}", arnumber);
                    eprintln!("IEEE: First visiting document page: {}", doc_url);
                    // Visit document page to establish session (ignore response)
                    let _ = self.request_pdf(&doc_url).await;
                }
            }

            match self.request_pdf(pdf_url).await {
                Ok(resp) => {
                    eprintln!("Response status: {}", resp.status());
                    if Self::is_pdf(&resp) {
                        eprintln!("PDF found at: {}", pdf_url);
                        return Self::stream_as_pdf(resp, doi);
                    } else {
                        eprintln!("Not a PDF, content-type: {:?}", resp.headers().get("content-type"));
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch {}: {:?}", pdf_url, e);
                }
            }
        }

        let encoded = utf8_percent_encode(doi, NON_ALPHANUMERIC).to_string();
        let target = format!("https://doi.org/{encoded}");
        eprintln!("Falling back to DOI redirect: {}", target);

        let resp = self.request_pdf(&target).await?;
        eprintln!("DOI redirect response status: {}", resp.status());
        if Self::is_pdf(&resp) {
            eprintln!("PDF found via DOI redirect");
            return Self::stream_as_pdf(resp, doi);
        }

        // Check if redirected to MDPI landing page
        let final_url = resp.url().as_str();
        eprintln!("Final URL after redirect: {}", final_url);

        // Handle MDPI
        if final_url.contains("www.mdpi.com") && !final_url.ends_with("/pdf") {
            let pdf_url = format!("{}/pdf", final_url.trim_end_matches('/'));
            eprintln!("MDPI detected, trying PDF URL: {}", pdf_url);

            // Small delay to avoid bot detection
            sleep(Duration::from_millis(500)).await;

            let pdf_resp = self.make_pdf_request(&pdf_url, final_url).await?;
            eprintln!("MDPI PDF response status: {}", pdf_resp.status());
            eprintln!("MDPI PDF content-type: {:?}", pdf_resp.headers().get("content-type"));

            if pdf_resp.status().is_success() &&
               (Self::is_pdf(&pdf_resp) || Self::is_likely_pdf(&pdf_resp)) {
                eprintln!("PDF found at MDPI PDF URL");
                return Self::stream_as_pdf(pdf_resp, doi);
            } else {
                eprintln!("MDPI response not recognized as PDF");
            }
        }

        // Handle Oxford University Press (OUP)
        if final_url.contains("academic.oup.com") && final_url.contains("/article/") {
            let doi_suffix = doi.split('/').last().unwrap_or("");
            let pdf_url = final_url
                .replace("/article/", "/article-pdf/")
                .trim_end_matches('/')
                .to_string() + "/" + doi_suffix + ".pdf";

            eprintln!("OUP detected, trying PDF URL: {}", pdf_url);

            let pdf_resp = self.make_pdf_request(&pdf_url, final_url).await?;
            eprintln!("OUP PDF response status: {}", pdf_resp.status());
            eprintln!("OUP PDF content-type: {:?}", pdf_resp.headers().get("content-type"));

            if pdf_resp.status().is_success() &&
               (Self::is_pdf(&pdf_resp) || Self::is_likely_pdf(&pdf_resp)) {
                eprintln!("PDF found at OUP PDF URL");
                return Self::stream_as_pdf(pdf_resp, doi);
            } else {
                eprintln!("OUP response not recognized as PDF");
            }
        }

        eprintln!("Could not locate PDF for DOI: {}", doi);
        Err(actix_web::error::ErrorBadGateway(
            "Could not locate PDF",
        ))
    }

    async fn request_pdf(&self, url: &str) -> actix_web::Result<Response> {
        self.client
            .get(url)
            .header(header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
            .header(header::ACCEPT_ENCODING, "gzip, deflate, br")
            .header(header::ACCEPT_LANGUAGE, "en-US,en;q=0.9")
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                AppleWebKit/537.36 (KHTML, like Gecko) \
                Chrome/120.0.0.0 Safari/537.36",
            )
            .header("Sec-Ch-Ua", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\"")
            .header("Sec-Ch-Ua-Mobile", "?0")
            .header("Sec-Ch-Ua-Platform", "\"Windows\"")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "none")
            .header("Sec-Fetch-User", "?1")
            .header("Upgrade-Insecure-Requests", "1")
            .send()
            .await
            .map_err(|e| {
                eprintln!("upstream request failed: {}", e);
                actix_web::error::ErrorBadGateway("Upstream request failed")
            })
    }

    async fn make_pdf_request(&self, url: &str, referer: &str) -> actix_web::Result<Response> {
        self.client
            .get(url)
            .header(header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
            .header(header::ACCEPT_ENCODING, "gzip, deflate, br")
            .header(header::ACCEPT_LANGUAGE, "en-US,en;q=0.9")
            .header(header::REFERER, referer)
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                AppleWebKit/537.36 (KHTML, like Gecko) \
                Chrome/120.0.0.0 Safari/537.36",
            )
            .header("Sec-Ch-Ua", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\"")
            .header("Sec-Ch-Ua-Mobile", "?0")
            .header("Sec-Ch-Ua-Platform", "\"Windows\"")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "same-origin")
            .header("Sec-Fetch-User", "?1")
            .header("Upgrade-Insecure-Requests", "1")
            .send()
            .await
            .map_err(|e| {
                eprintln!("PDF request failed: {}", e);
                actix_web::error::ErrorBadGateway("PDF request failed")
            })
    }

    fn is_pdf(resp: &Response) -> bool {
        resp.headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.to_ascii_lowercase().starts_with("application/pdf"))
            .unwrap_or(false)
    }

    fn is_likely_pdf(resp: &Response) -> bool {
        // Check for PDF-like content types or URL patterns
        let content_type = resp.headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.to_ascii_lowercase());

        if let Some(ct) = content_type {
            // Some servers return these for PDFs
            if ct.contains("application/pdf") ||
               ct.contains("application/octet-stream") ||
               ct.contains("binary/octet-stream") {
                return true;
            }
        }

        // Check if URL ends with .pdf
        resp.url().as_str().ends_with("/pdf") || resp.url().as_str().ends_with(".pdf")
    }

    fn stream_as_pdf(resp: Response, doi: &str) -> actix_web::Result<HttpResponse> {
        if !resp.status().is_success() {
            return Err(actix_web::error::ErrorBadGateway(
                "Upstream returned non-success for PDF",
            ));
        }

        let mut builder = HttpResponse::Ok();
        builder.append_header((actix_header::CONTENT_TYPE, "application/pdf"));

        let safe = doi.replace('/', "_");
        builder.append_header((
            actix_header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{safe}.pdf\""),
        ));

        let stream = resp
            .bytes_stream()
            .map_ok(actix_web::web::Bytes::from)
            .map_err(|_| actix_web::error::ErrorBadGateway("Failed to stream PDF"));

        Ok(builder.streaming(stream))
    }
}
