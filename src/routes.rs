use axum::{body::Body, extract::{Path, Request}, response::{Html, IntoResponse, Response}, Json};
use tokio::fs::read_to_string;
use tower_http::services::ServeFile;

use crate::{consts::{DEFAULT_CSS, INDEX_HTML, NO_INSTRUMENTS_HTML}, utils::{get_instruments, parse_referer}};

pub async fn instruments_list_handler() -> impl IntoResponse {
    Json(get_instruments())
}

pub async fn default_css_handler() -> impl IntoResponse {
    Response::builder()
        .header("Content-Type", "text/css")
        .body(DEFAULT_CSS.to_string())
        .unwrap()
}

pub async fn static_files_handler(Path(file_path): Path<String>, req: Request<Body>) -> impl IntoResponse {
    let raw_referer = req
        .headers()
        .get("referer")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("/")
        .to_string();

    let referer = parse_referer(raw_referer.clone());
    let file_path = format!("./sites/{referer}/assets/{file_path}");

    ServeFile::new(file_path)
        .try_call(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        .into_response()
}

pub async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}

pub async fn instrument_handler(Path(instrument): Path<String>) -> Html<String> {
    Html(
        read_to_string(format!("./sites/{instrument}/index.html"))
        .await
        .unwrap_or(NO_INSTRUMENTS_HTML.to_string())
    )
}