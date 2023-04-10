#[derive(sqlx::FromRow)]
pub struct Message {
    pub id: i32,
    pub payload: String,
    pub created_at: chrono::NaiveDateTime,
}

pub struct UpdateMessage {
    pub id: i32,
    pub payload: String,
}
