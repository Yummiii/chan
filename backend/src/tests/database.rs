use chrono::Utc;
use sqlx::MySqlPool;

use crate::{
    config::Config,
    database::{
        Pools, boards::BoardsRepository, images::ImagesRepository, posts::PostsRepository,
        users::UsersRepository,
    },
    models::{board::Board, image::Image, post::Post, user::User},
};

#[tokio::test]
async fn connection() {
    let config = Config::get();
    assert!(
        config.database.url.starts_with("mysql://"),
        "O banco deve ser MySQL ou MariaDB"
    );

    let database = Pools::init(&config.database.url).await;
    assert!(
        database.is_ok(),
        "Coneão com o banco de dados falhou após 5 tentativas"
    )
}

#[sqlx::test]
async fn create_user(pool: MySqlPool) {
    let users_repo = UsersRepository::new(pool);

    let user = users_repo
        .create_user(&User {
            id: "aa123".into(),
            pass_hash: "hash".into(),
            username: "username".into(),
        })
        .await
        .unwrap();

    assert_eq!(user.id, "aa123");
    assert_eq!(user.pass_hash, "hash");
    assert_eq!(user.username, "username");
}

#[sqlx::test]
async fn create_board_and_post(pool: MySqlPool) {
    let posts_repo = PostsRepository::new(pool.clone());
    let board_repository = BoardsRepository::new(pool);

    let board = board_repository
        ._create(&Board {
            id: 0,
            name: "board1".into(),
            slug: "b".into(),
        })
        .await
        .unwrap();

    assert_eq!(board.name, "board1");
    assert_eq!(board.slug, "b");
    assert!(board.id > 0);

    let timestamp = Utc::now().timestamp();
    let post = posts_repo
        .create(&Post {
            id: 0,
            board_id: board.id,
            thread_id: None,
            user_id: None,
            content: "Hello, world!".into(),
            image_id: None,
            created_at: timestamp,
        })
        .await
        .unwrap();

    assert_eq!(post.board_id, board.id);
    assert_eq!(post.thread_id, None);
    assert_eq!(post.user_id, None);
    assert_eq!(post.content, "Hello, world!");
    assert_eq!(post.image_id, None);
    assert_eq!(post.created_at, timestamp);
    assert!(post.id > 0);
}

#[sqlx::test]
async fn create_image(pool: MySqlPool) {
    let images_repo = ImagesRepository::new(pool);

    images_repo
        .create(&Image {
            id: "image_id".into(),
            data: vec![1, 2, 3],
            mime: "image/png".into(),
        })
        .await
        .unwrap();

    let image = images_repo.get_by_id("image_id").await.unwrap();

    assert_eq!(image.id, "image_id");
    assert_eq!(image.data, vec![1, 2, 3]);
    assert_eq!(image.mime, "image/png");
}
