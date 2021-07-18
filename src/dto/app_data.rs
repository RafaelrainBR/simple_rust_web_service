use crate::dto::User;

pub struct CustomAppData {
    pub local_database: Vec<User>,
}

impl CustomAppData {
    pub fn new(local_database: Vec<User>) -> Self {
        Self { local_database }
    }
}
