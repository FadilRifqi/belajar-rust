use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::user::User;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub namespace: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        client.use_ns("surreal").use_db("crud").await?;
        Ok(Database {
            client,
            namespace: String::from("surreal"),
            db_name: String::from("crud"),
        })
    }
    pub async fn get_all_user(&self) -> Option<Vec<User>> {
        let res = self.client.select("crud").await;
        match res {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }
    pub async fn add_user(&self, user: User) -> Option<User> {
        let created_user = self
            .client
            .create(("user", user.uuid.clone()))
            .content(user)
            .await;

        match created_user {
            Ok(data) => data,
            Err(_) => None,
        }
    }
}
