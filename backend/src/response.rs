use poem_openapi::{
    ApiResponse, Object,
    payload::Json,
    types::{ParseFromJSON, ToJSON},
};

pub trait ApiData: ParseFromJSON + ToJSON + Send + Sync {}
impl<T: ParseFromJSON + ToJSON + Send + Sync> ApiData for T {}

#[derive(Object)]
pub struct ResponseObject<T: ApiData> {
    msg: String,
    data: Option<T>,
    #[oai(rename = "isError")]
    is_error: bool,
}

impl<T: ApiData> ResponseObject<T> {
    pub fn ok(data: T) -> Self {
        Self {
            msg: "OK".to_string(),
            data: Some(data),
            is_error: false,
        }
    }

    pub fn bad_request(msg: String) -> Self {
        Self {
            msg,
            data: None,
            is_error: true,
        }
    }

    pub fn internal_server_error(msg: String) -> Self {
        Self {
            msg,
            data: None,
            is_error: true,
        }
    }
}

#[derive(ApiResponse)]
pub enum Response<T: ApiData> {
    #[oai(status = 200)]
    Ok(Json<ResponseObject<T>>),
    #[oai(status = 400)]
    BadRequest(Json<ResponseObject<T>>),
    #[oai(status = 500)]
    InternalServerError(Json<ResponseObject<T>>),
}

pub fn ok<T: ApiData>(data: T) -> Response<T> {
    Response::Ok(Json(ResponseObject::ok(data)))
}

pub fn bad_request<T: ApiData>(msg: impl Into<String>) -> Response<T> {
    Response::BadRequest(Json(ResponseObject::bad_request(msg.into())))
}

pub fn internal_error<T: ApiData>(msg: impl Into<String>) -> Response<T> {
    Response::InternalServerError(Json(ResponseObject::internal_server_error(msg.into())))
}

pub fn handle_db_error<T: ApiData>(
    err: sqlx::Error,
    nfm: Option<impl Into<String>>,
) -> Response<T> {
    match err {
        sqlx::Error::RowNotFound => match nfm {
            Some(message) => bad_request(message.into()),
            None => {
                eprintln!("Row not found");
                bad_request("Row not found")
            }
        },
        e => {
            eprintln!("Database error: {}", e);
            internal_error("Database error")
        }
    }
}
