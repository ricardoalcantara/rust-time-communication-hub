#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub external_id: String,
    pub created_at: chrono::NaiveDateTime,
}
