use actix_web::{get, middleware, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Param {
    pub param: Option<String>,
}

#[get("/")]
async fn index(param: web::Query<Param>) -> HttpResponse {
    log::debug!("param: {:?}", param);

    if param.param.is_none() {
        return HttpResponse::Ok().body(format!("Start!"));
    }

    HttpResponse::Ok().body(format!("{:?}", param))
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("404")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
