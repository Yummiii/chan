use poem_openapi::Object;

#[derive(Debug, Object)]
pub struct Token {
    /// The JWT token to be used in the Authorization header
    pub token: String,
}

impl Token {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}
