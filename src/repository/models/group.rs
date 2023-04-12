#[derive(sqlx::FromRow)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

impl Group {
    pub fn new(name: String) -> Group {
        Group { id: 0, name }
    }
}

pub struct UpdateGroup {
    pub id: i32,
    pub name: String,
}
