use serde::Deserialize;

#[derive(Deserialize)]
pub struct SseParams {
    pub token: String,
}
