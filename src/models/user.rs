// user.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct UserRequest {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(range(min = 18, max = 100))]
    pub age: i32,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateUserURL {
    #[validate(length(min = 3))]
    pub uuid: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub age: i32,
}

impl User {
    pub fn new(uuid: String, name: String, email: String, password: String, age: i32) -> User {
        User {
            uuid,
            name,
            email,
            password,
            age,
        }
    }
}
