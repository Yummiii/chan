use super::ApiTags;
use crate::{
    database::{users::User, Pools}, hdbe, models::token::Token, response::{bad_request, ok, Response}, utils::password::{hash_password, verify_password}
};
use cuid2::cuid;
use poem::web::Data;
use poem_openapi::{Object, OpenApi, payload::Json};

pub struct AuthController;

#[derive(Object)]
struct UserCredentials {
    username: String,
    password: String,
}

#[OpenApi(prefix_path = "/auth", tag = "ApiTags::Auth")]
impl AuthController {
    /// Authenticates a user
    #[oai(path = "/login", method = "post", operation_id = "auth-login")]
    async fn login(
        &self,
        pools: Data<&Pools>,
        credentials: Json<UserCredentials>,
    ) -> Response<Token> {
        let user = hdbe!(
            pools.users.get_by_username(&credentials.username).await,
            "Invalid username or password"
        );
        if !verify_password(&credentials.password, &user.pass_hash) {
            return bad_request("Invalid username or password");
        }

        ok(Token::new("todo"))
    }

    /// Registers a new user
    #[oai(path = "/register", method = "post", operation_id = "auth-register")]
    async fn register(
        &self,
        pools: Data<&Pools>,
        credentials: Json<UserCredentials>,
    ) -> Response<Token> {
        let user_db = pools.users.get_by_username(&credentials.username).await;
        if user_db.is_ok() {
            return bad_request("Username is already taken");
        }

        let _user = hdbe!(
            pools
                .users
                .create_user(&User {
                    id: cuid(),
                    username: credentials.username.clone(),
                    pass_hash: hash_password(&credentials.password),
                })
                .await
        );

        ok(Token::new("todo"))
    }
}
