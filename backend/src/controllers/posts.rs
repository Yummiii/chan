use super::ApiTags;
use crate::{
    database::Pools,
    hdbe,
    models::{image::Image, post::Post},
    response::{Response, bad_request, ok},
};
use chrono::Utc;
use cuid2::cuid;
use poem::web::Data;
use poem_openapi::{Multipart, OpenApi, types::multipart::Upload};

pub struct PostsController;

#[derive(Multipart)]
struct NewPost {
    /// The board slug where the post will be created.
    board: String,
    /// The post text content.
    content: String,
    /// The post thread, if null or 0, the post will be a new thread.
    thread: Option<u64>,
    /// Image attached to the post.
    image: Option<Upload>,
}

#[OpenApi(prefix_path = "/posts", tag = "ApiTags::Posts")]
impl PostsController {
    /// Creates a new post
    #[oai(path = "/", method = "post", operation_id = "posts-create")]
    async fn create(&self, pools: Data<&Pools>, mut post: NewPost) -> Response<Post> {
        let board = hdbe!(
            pools.boards.get_by_slug(&post.board).await,
            "Board not found"
        );

        if post.thread == Some(0) {
            post.thread = None;
        }

        if let Some(thread) = post.thread {
            hdbe!(pools.posts.get_by_id(thread).await, "Thread not found");
        }

        let image_id = if let Some(image) = post.image {
            if let Some(content_type) = image.content_type() {
                if !content_type.starts_with("image/") {
                    return bad_request("Invalid image type");
                }

                let image = Image {
                    mime: content_type.to_owned(),
                    data: image.into_vec().await.unwrap(),
                    id: cuid(),
                };

                hdbe!(pools.images.create(&image).await);

                Some(image.id)
            } else {
                None
            }
        } else {
            None
        };

        let post = Post {
            id: 0,
            content: post.content,
            created_at: Utc::now().timestamp(),
            board_id: board.id,
            user_id: None,
            thread_id: post.thread,
            image_id,
        };

        ok(hdbe!(pools.posts.create(&post).await))
    }
}
