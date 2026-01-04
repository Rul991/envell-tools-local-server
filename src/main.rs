use std::env::args;

use axum::{routing::get, serve, Router};
use tokio::{main, net::TcpListener};

use crate::routes::{default_css_handler, index_handler, instrument_handler, instruments_list_handler, static_files_handler};
use crate::utils::print_instruments;

mod utils;
mod consts;
mod routes;

const DEFAULT_PORT: u16 = 3000;

#[main]
async fn main() {
    let argv: &Vec<String> = &args().collect();
    let default_value = format!("{DEFAULT_PORT}");

    let arg_port = argv.get(1).unwrap_or(&default_value);
    let port = arg_port.parse::<u16>().unwrap_or(DEFAULT_PORT);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/assets/{*file}", get(static_files_handler))
        .route("/instruments", get(instruments_list_handler))
        .route("/{instrument}", get(instrument_handler))
        .route("/default.css", get(default_css_handler));
        
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    
    let addr = format!("127.0.0.1:{port}");
    println!("Сервер запущен по адресу: http://{addr}\n");
    print_instruments(&addr);

    serve(
        listener, 
        app
            .into_make_service()
    ).await.unwrap()
}