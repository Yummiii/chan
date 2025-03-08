use config::Config;
use database::Database;
use poem::{Route, Server, get, handler, listener::TcpListener};

mod config;
mod database;
mod controllers;

#[handler]
fn hello() -> String {
    format!("Hello world")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();

    let config = Config::get();
    let db = Database::init(&config.database.url).await;

    let app = Route::new().at("hello", get(hello));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
