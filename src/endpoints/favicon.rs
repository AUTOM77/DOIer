use actix_web::{get, HttpResponse, Responder};

const FAVICON_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32">
  <defs>
    <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
    </linearGradient>
  </defs>
  <rect width="32" height="32" rx="6" fill="url(#gradient)"/>
  <text x="16" y="23" font-family="Arial, sans-serif" font-size="20" font-weight="bold" text-anchor="middle" fill="white">D</text>
</svg>"#;

#[get("/favicon.ico")]
pub async fn favicon_ico() -> impl Responder {
    // Return SVG favicon as browsers now support SVG favicons
    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .append_header(("Cache-Control", "public, max-age=86400"))
        .body(FAVICON_SVG)
}

#[get("/favicon.svg")]
pub async fn favicon_svg() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .append_header(("Cache-Control", "public, max-age=86400"))
        .body(FAVICON_SVG)
}