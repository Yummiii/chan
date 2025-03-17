use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub username: String,
    pub id: String,
}

impl From<User> for UserClaims {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            id: user.id,
        }
    }
}
