use actix_web::{
    get, patch, post, web::Data, web::Json, web::Path, App, HttpResponse, HttpServer, Responder,
};
use uuid::Uuid;
use validator::Validate;
mod db;
mod models;
use crate::db::database::Database;
use crate::models::user::{UpdateUserURL, User, UserRequest};
#[get("/users")]
async fn get_all_user(db: Data<Database>) -> impl Responder {
    let users = db.get_all_user().await;
    match users {
        Some(data) => HttpResponse::Ok().json(data),
        None => HttpResponse::Ok().body("No data found"),
    }
}

#[get("/user/{uuid}")]
async fn get_user(db: Data<Database>) -> impl Responder {
    HttpResponse::Ok().body("User")
}

#[post("/user")]
async fn post(body: Json<UserRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let user_name = body.name.clone();
            let user_email = body.email.clone();
            let user_age = body.age.clone();
            let user_password: String = body.password.clone();
            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_user = db
                .add_user(User::new(
                    String::from(new_uuid),
                    user_name,
                    user_email,
                    user_password,
                    user_age,
                ))
                .await;

            match new_user {
                Some(created) => HttpResponse::Ok().body(format!(
                    "Name: {}, Email: {}, Age: {}",
                    created.name, created.email, created.age
                )),
                None => HttpResponse::Ok().body("Error"),
            }
        }
        Err(_) => HttpResponse::Ok().body("Error"),
    }
}

#[patch("/user/{uuid}")]
async fn patch(update_user_url: Path<UpdateUserURL>) -> impl Responder {
    let uuid = update_user_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("UUID: {}", uuid))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Failed to connect to database");
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_all_user)
            .service(get_user)
            .service(post)
            .service(patch)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
