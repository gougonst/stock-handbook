use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserModel {
    username: String, 
    password: String, 
}

impl UserModel {
    pub fn new(username: String, password: String) -> Self {
        UserModel {
            username, 
            password, 
        }
    }
}

pub trait User: Send + Sync {
    fn username(&self) -> &str;
    fn password(&self) -> &str;
}

impl User for UserModel {
    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }
}
