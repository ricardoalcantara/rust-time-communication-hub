#[derive(sqlx::FromRow)]
pub struct Group {
    pub id: i32,
    pub name: String,
}
