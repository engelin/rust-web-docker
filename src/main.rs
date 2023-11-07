pub mod hello_world;

use actix_web::{get, middleware, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

use hello_world::param::Param;

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryParam {
    pub param: Option<String>,
}

#[get("/")]
async fn index(param_query: web::Query<QueryParam>) -> HttpResponse {
    log::debug!("param: {:?}", param_query);

    if param_query.param.is_none() {
        return HttpResponse::Ok().body(format!("Start!"));
    }

    let mut param = Param::new();
    let (is_valid, msg) = param.set_param(param_query.param.clone().unwrap());
    if !is_valid {
        return HttpResponse::BadRequest().body(msg.unwrap());
    }
    log::debug!("param_vec: {:?}", param);

    HttpResponse::Ok().body(format!("{:?}", param))
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("404")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));

    let port = match std::env::var("PORT") {
        Ok(port) => port,
        _ => String::from("8080"),
    };

    let address = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .default_service(web::route().to(not_found))
    })
    .bind(address.clone())?
    .run()
    .await
}
