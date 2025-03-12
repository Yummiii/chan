use config::Config;
use controllers::get_service;
use database::Pools;
use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};

mod config;
mod controllers;
mod database;
mod macros;
mod response;
mod models;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();

    let config = Config::get();
    let db = Pools::init(&config.database.url).await;

    let api_service = get_service();
    let ui = api_service.swagger_ui();

    let route = Route::new()
        .nest("/", api_service)
        .nest("/swagger", ui)
        .with(Cors::new())
        .data(db);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(route)
        .await
}
