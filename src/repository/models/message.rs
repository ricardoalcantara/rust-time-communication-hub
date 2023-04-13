#[derive(sqlx::FromRow)]
pub struct Message {
    pub id: i32,
    pub payload: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Message {
    pub fn new(payload: String) -> Message {
        Message {
            id: 0,
            payload,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

pub struct UpdateMessage {
    pub id: i32,
    pub payload: String,
}
