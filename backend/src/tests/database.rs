use crate::{config::Config, database::Pools};

#[tokio::test]
async fn connection() {
    let config = Config::get();
    assert!(config.database.url.starts_with("mysql://"), "O banco deve ser MySQL ou MariaDB");

    let database = Pools::init(&config.database.url).await;
    assert!(database.is_ok(), "Conexão com o banco de dados falhou após 5 tentativas")
}
