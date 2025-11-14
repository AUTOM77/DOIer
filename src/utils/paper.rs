use actix_web::{http::header as actix_header, HttpResponse};
use futures_util::TryStreamExt as _;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{header, redirect::Policy, Client, Response};
use std::time::Duration;

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
            .cookie_store(true)  // Enable cookie storage for IEEE session handling
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
        if final_url.contains("www.mdpi.com") && !final_url.ends_with("/pdf") {
            let pdf_url = format!("{}/pdf", final_url.trim_end_matches('/'));
            eprintln!("MDPI detected, trying PDF URL: {}", pdf_url);
            let pdf_resp = self.request_pdf(&pdf_url).await?;
            if Self::is_pdf(&pdf_resp) {
                eprintln!("PDF found at MDPI PDF URL");
                return Self::stream_as_pdf(pdf_resp, doi);
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
            .header(header::ACCEPT, "application/pdf")
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                AppleWebKit/537.36 (KHTML, like Gecko) \
                Chrome/120.0.0.0 Safari/537.36",
            )
            .header(
                "Sec-CH-UA",
                "\"Chromium\";v=120, \"Google Chrome\";v=120, \"Not:A-Brand\";v=99",
            )
            .header("Sec-CH-UA-Mobile", "?0")
            .header("Sec-CH-UA-Platform", "\"Windows\"")
            .send()
            .await
            .map_err(|e| {
                eprintln!("upstream request failed: {}", e);
                actix_web::error::ErrorBadGateway("Upstream request failed")
            })
    }

    fn is_pdf(resp: &Response) -> bool {
        resp.headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.to_ascii_lowercase().starts_with("application/pdf"))
            .unwrap_or(false)
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
