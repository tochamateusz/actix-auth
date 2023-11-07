use actix_web::{get, App, HttpRequest, HttpServer, Responder};

#[get("/")]
async fn get_users(_req: HttpRequest) -> impl Responder {
    "Get Users"
}

#[get("/login")]
async fn login(_req: HttpRequest) -> impl Responder {
    "Login"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_users).service(login))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
