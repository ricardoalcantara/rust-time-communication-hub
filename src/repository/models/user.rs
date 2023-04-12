#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub external_id: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn new(external_id: String) -> User {
        User {
            id: 0,
            external_id,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
