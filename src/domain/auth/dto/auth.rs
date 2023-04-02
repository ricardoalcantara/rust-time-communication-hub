use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    #[serde(rename = "type")]
    pub token_type: String,
}
