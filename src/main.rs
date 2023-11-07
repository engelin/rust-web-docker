use actix_web::{get, middleware, web, App, HttpServer, HttpResponse, HttpRequest};

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    let query_string = req.query_string();

    let query_params: Vec<(&str, &str)> = query_string
        .split('&')
        .map(|param| {
            let parts: Vec<&str> = param.splitn(2, '=').collect();
            (parts[0], parts.get(1).copied().unwrap_or(""))
        })
        .collect();

    HttpResponse::Ok().body(format!("{:?}", query_params))
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
