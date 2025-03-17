use crate::models::user::User;
use jwt_simple::{
    claims::Claims,
    prelude::{Duration, HS256Key, MACLike},
};

pub fn create_token(user: User, key: &str) -> String {
    let key = HS256Key::from_bytes(key.as_bytes());
    let mut claims = Claims::with_custom_claims(user, Duration::from_days(90));
    claims.create_nonce();
    key.authenticate(claims).unwrap()
}
