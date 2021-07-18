use serde::Serialize;
use std::fmt::Display;

#[derive(Clone, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub idade: u8,
}

impl User {
    pub fn new(id: u32, name: &str, idade: u8) -> Self {
        Self {
            id,
            name: name.to_owned(),
            idade,
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, name:{}, idade:{}",
            self.id, self.name, self.idade
        )
    }
}
