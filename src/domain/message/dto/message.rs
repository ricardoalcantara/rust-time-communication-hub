use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct NofityMessage {
    pub users: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
    #[validate(length(min = 1, message = "Payload cannot be empty"))]
    pub payload: String,
}
