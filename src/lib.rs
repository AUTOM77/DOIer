mod config;
mod endpoints;
mod utils;
mod templates;

use actix_web::{App, HttpServer};

async fn actix(
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let listen = format!("{}:{}", "0.0.0.0", port);

    let server = HttpServer::new(move || {
        App::new()
            .configure(endpoints::routes::init)
    });

    println!("Starting server at {}", listen);
    let server = server.bind(listen)?;
    server.run().await?;

    Ok(())
}

pub fn interface(
    _port: Option<u16>,
) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let port = _port.unwrap_or(8080);
    println!("* [Debug] {}", port);

    let _ = rt.block_on(actix(port));
    Ok(())
}
