use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderName, HeaderValue};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::config::Config;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod config;
mod core;
pub mod logger;

pub async fn run(config: Config) -> anyhow::Result<()> {
    log::info!("Run HTTP server");

    let router = Router::new().route("/", get(handler)).layer(
        ServiceBuilder::new()
            .layer(NormalizePathLayer::trim_trailing_slash())
            .layer(DefaultBodyLimit::max(config.http.upload_file_limit))
            .layer(CompressionLayer::new())
            .layer(SetResponseHeaderLayer::if_not_present(
                HeaderName::from_static("x-version"),
                HeaderValue::from_static(VERSION),
            )),
    );

    let addr = SocketAddr::new(IpAddr::from_str(&config.http.host)?, config.http.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    log::info!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}

async fn handler() -> impl IntoResponse {
    Html("<h1>Hello, bro gear!</h1>")
}
