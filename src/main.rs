use actix_web::{App, HttpServer};

mod controllers;
use controllers::handlers::{add_user, delete_user_by_id, get_users, get_users_by_id, login};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_users)
            .service(get_users_by_id)
            .service(add_user)
            .service(delete_user_by_id)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
