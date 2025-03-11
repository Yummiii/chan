use super::ApiTags;
use crate::database::Pools;
use poem::{
    Result,
    error::{NotFound, NotFoundError},
    web::{Data, Path},
};
use poem_openapi::{
    OpenApi,
    payload::{Attachment, AttachmentType, Response},
};

pub struct ImagesController;

#[OpenApi(prefix_path = "/i", tag = "ApiTags::Images")]
impl ImagesController {
    /// Get image by id
    #[oai(path = "/:id", method = "get")]
    async fn get_image(
        &self,
        id: Path<String>,
        pools: Data<&Pools>,
    ) -> Result<Response<Attachment<Vec<u8>>>> {
        match pools.images.get_by_id(&id.0).await {
            Ok(img) => {
                let attachment = Attachment::new(img.data).attachment_type(AttachmentType::Inline);
                Ok(Response::new(attachment).header("Content-Type", img.mime).header("Cache-Control", "max-age=2592000, private"))
            }
            Err(_) => Err(NotFound(NotFoundError)),
        }
    }
}
