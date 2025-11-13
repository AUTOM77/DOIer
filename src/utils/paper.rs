use actix_web::{http::header as actix_header, HttpResponse};
use futures_util::TryStreamExt as _;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{header, redirect::Policy, Client, Response};
use scraper::{Html, Selector};
use std::time::Duration;

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
            .build()
            .expect("reqwest client");
        Self { client }
    }

    pub async fn fetch_pdf_httpresponse(&self, doi: &str) -> actix_web::Result<HttpResponse> {
        let encoded = utf8_percent_encode(doi, NON_ALPHANUMERIC).to_string();
        let target = format!("https://doi.org/{encoded}");

        let resp = self.request_pdf(&target).await?;
        if Self::is_pdf(&resp) {
            return Self::stream_as_pdf(resp, doi);
        }

        let body = resp
            .text()
            .await
            .map_err(|_| actix_web::error::ErrorBadGateway("Unable to read landing page"))?;
        let base = reqwest::Url::parse(&target).ok();

        if let Some(pdf_url) = Self::extract_pdf_url(&body, base.as_ref()) {
            let resp2 = self
                .client
                .get(&pdf_url)
                .header(
                    header::USER_AGENT,
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                    AppleWebKit/537.36 (KHTML, like Gecko) \
                    Chrome/120.0.0.0 Safari/537.36",
                )
                .send()
                .await
                .map_err(|_| actix_web::error::ErrorBadGateway("Failed to fetch resolved PDF"))?;

            if Self::is_pdf(&resp2) {
                return Self::stream_as_pdf(resp2, doi);
            }
        }

        Err(actix_web::error::ErrorBadGateway(
            "Could not locate PDF after negotiation + parse",
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

    fn extract_pdf_url(html: &str, base: Option<&reqwest::Url>) -> Option<String> {
        let doc = Html::parse_document(html);

        if let Ok(sel) = Selector::parse("iframe") {
            for node in doc.select(&sel) {
                if let Some(src) = node.value().attr("src") {
                    if let Some(url) = Self::resolve(base, src) {
                        if url.to_ascii_lowercase().contains(".pdf") {
                            return Some(url);
                        }
                    }
                }
            }
        }

        if let Ok(sel) = Selector::parse("a") {
            for node in doc.select(&sel) {
                if let Some(href) = node.value().attr("href") {
                    if href.to_ascii_lowercase().contains(".pdf") {
                        if let Some(url) = Self::resolve(base, href) {
                            return Some(url);
                        }
                    }
                }
            }
        }
        None
    }

    fn resolve(base: Option<&reqwest::Url>, candidate: &str) -> Option<String> {
        if let Ok(u) = reqwest::Url::parse(candidate) {
            return Some(u.to_string());
        }
        if let Some(b) = base {
            if let Ok(u) = b.join(candidate) {
                return Some(u.to_string());
            }
        }
        None
    }

    fn stream_as_pdf(resp: Response, doi: &str) -> actix_web::Result<HttpResponse> {
        if !resp.status().is_success() {
            return Err(actix_web::error::ErrorBadGateway(
                "Upstream returned non-success for PDF",
            ));
        }

        let mut builder = HttpResponse::Ok();
        builder.append_header((actix_header::CONTENT_TYPE, "application/pdf"));

        if let Some(cd) = resp
            .headers()
            .get(header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
        {
            builder.append_header((actix_header::CONTENT_DISPOSITION, cd.to_string()));
        } else {
            let safe = doi.replace('/', "_");
            builder.append_header((
                actix_header::CONTENT_DISPOSITION,
                format!("inline; filename=\"{safe}.pdf\""),
            ));
        }

        let stream = resp
            .bytes_stream()
            .map_ok(actix_web::web::Bytes::from)
            .map_err(|_| actix_web::error::ErrorBadGateway("Failed to stream PDF"));

        Ok(builder.streaming(stream))
    }
}
