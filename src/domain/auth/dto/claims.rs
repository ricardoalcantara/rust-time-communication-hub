use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub jti: String,
    pub sub: String,
    pub exp: i64,
}
