use actix_web::{get,post,patch, App, HttpResponse, HttpServer,Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyResponse {
    message: String,
    status: u32
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/api")]
async fn api() -> impl Responder {
    HttpResponse::Ok().body("rest api!")
}

#[post("/")]
async  fn post() -> impl Responder{
    let response = MyResponse {
        message: "post request".to_string(),
        status: 200,
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index).service(api).service(post)
    })
    .bind("127.0.0.1:8080")?.run().await
}
    
