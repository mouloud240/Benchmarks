use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Greeting {
    #[serde(rename = "hello word v1 ")]
    message: &'static str,
}

#[get("/greetings")]
async fn greetings() -> impl Responder {
    let g = Greeting { message: "Hello, World!" };
    HttpResponse::Ok().json(g)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(web::scope("/v1").service(greetings))
            )
    })
    .bind(("0.0.0.0", 8001))?
    .run()
    .await
}
