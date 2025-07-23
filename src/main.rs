use axum::{routing::get, serve, Router};
use tokio::{main, net::TcpListener};

use crate::routes::{default_css_handler, index_handler, instrument_handler, instruments_list_handler, static_files_handler};
use crate::utils::print_instruments;

mod utils;
mod consts;
mod routes;

#[main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/assets/{*file}", get(static_files_handler))
        .route("/instruments", get(instruments_list_handler))
        .route("/{instrument}", get(instrument_handler))
        .route("/default.css", get(default_css_handler));
        
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    let addr = "127.0.0.1:3000";
    println!("Сервер запущен по адресу: http://{addr}\n");
    print_instruments(addr);

    serve(
        listener, 
        app
            .into_make_service()
    ).await.unwrap()
}