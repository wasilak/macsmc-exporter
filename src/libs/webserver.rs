use axum::Router;
use axum::body::Body;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use prometheus::{Encoder, TextEncoder};

pub async fn metrics_handler() -> impl IntoResponse {
    let metric_families = prometheus::gather();
    let encoder = TextEncoder::new();

    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap()
}

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new().route("/metrics", get(metrics_handler));

    let port = 8726;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    Ok(())
}
