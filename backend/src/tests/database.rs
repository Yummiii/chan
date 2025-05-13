use crate::{
    database::{
        Pools, boards::BoardsRepository, images::ImagesRepository, posts::PostsRepository,
        users::UsersRepository,
    },
    models::{board::Board, image::Image, post::Post, user::User},
};
use chan_config::Config;
use chrono::Utc;
use sqlx::MySqlPool;

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
        "Conexão com o banco de dados falhou após 5 tentativas"
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
async fn create_board(pool: MySqlPool) {
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
}

#[sqlx::test(fixtures("boards"))]
async fn create_anon_post(pool: MySqlPool) {
    let posts_repo = PostsRepository::new(pool.clone());

    let timestamp = Utc::now().timestamp();
    let post = posts_repo
        .create(&Post {
            id: 0,
            board_id: 1,
            thread_id: None,
            user_id: None,
            content: "Hello, world!".into(),
            image_id: None,
            created_at: timestamp,
        })
        .await
        .unwrap();

    assert_eq!(post.board_id, 1);
    assert_eq!(post.thread_id, None);
    assert_eq!(post.user_id, None);
    assert_eq!(post.content, "Hello, world!");
    assert_eq!(post.image_id, None);
    assert_eq!(post.created_at, timestamp);
    assert!(post.id > 0);
}

#[sqlx::test(fixtures("boards", "users"))]
async fn create_post(pool: MySqlPool) {
    let posts_repo = PostsRepository::new(pool.clone());

    let timestamp = Utc::now().timestamp();
    let post = posts_repo
        .create(&Post {
            id: 0,
            board_id: 1,
            thread_id: None,
            user_id: Some("abc123".into()),
            content: "Hello, world!".into(),
            image_id: None,
            created_at: timestamp,
        })
        .await
        .unwrap();

    assert_eq!(post.board_id, 1);
    assert_eq!(post.thread_id, None);
    assert_eq!(post.user_id, Some("abc123".into()));
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

#[sqlx::test(fixtures("boards"))]
async fn get_boards(pool: MySqlPool) {
    let boards_repo = BoardsRepository::new(pool);
    let boards = boards_repo.get_all().await.unwrap();

    assert_eq!(boards.len(), 3);
    assert_eq!(boards[0].name, "board");
    assert_eq!(boards[0].slug, "b");

    assert_eq!(boards[1].name, "anime");
    assert_eq!(boards[1].slug, "a");

    assert_eq!(boards[2].name, "supernatural");
    assert_eq!(boards[2].slug, "x");
}

#[sqlx::test(fixtures("boards"))]
async fn get_board_by_slug(pool: MySqlPool) {
    let boards_repo = BoardsRepository::new(pool);
    let board = boards_repo.get_by_slug("b").await.unwrap();

    assert_eq!(board.name, "board");
    assert_eq!(board.slug, "b");
}

#[sqlx::test(fixtures("boards", "images", "users", "posts"))]
async fn get_board_root_posts(pool: MySqlPool) {
    let posts_repo = PostsRepository::new(pool);
    let posts = posts_repo.get_root_by_board(1).await.unwrap();

    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].board_id, 1);
    assert_eq!(posts[0].thread_id, None);
    assert_eq!(posts[0].image_id, Some("img123".into()))
}

#[sqlx::test(fixtures("boards", "images", "users", "posts"))]
async fn get_thread_posts(pool: MySqlPool) {
    let posts_repo = PostsRepository::new(pool);
    let posts = posts_repo.get_thread(1).await.unwrap();

    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].board_id, 1);
    assert_eq!(posts[0].thread_id, None);
    assert_eq!(posts[0].image_id, Some("img123".into()));

    assert_eq!(posts[1].board_id, 1);
    assert_eq!(posts[1].thread_id, Some(1));
    assert_eq!(posts[1].image_id, None);
}
