use crate::models::user_claims::UserClaims;
use jwt_simple::{
    claims::Claims,
    prelude::{Duration, HS256Key, MACLike},
};

pub fn create_token(user: impl Into<UserClaims>, key: &str) -> String {
    let key = HS256Key::from_bytes(key.as_bytes());
    let mut claims = Claims::with_custom_claims(user.into(), Duration::from_days(90));
    claims.create_nonce();
    key.authenticate(claims).unwrap()
}

pub fn _verify_token(token: &str, key: &str) -> Result<UserClaims, jwt_simple::Error> {
    let key = HS256Key::from_bytes(key.as_bytes());
    let claims = key.verify_token(token, None)?;
    Ok(claims.custom)
}
