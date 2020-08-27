use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware};
use listenfd::ListenFd;
use std::env;
use dotenv::dotenv;

mod routes;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .configure(routes::init)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(server_url.clone())?
    };

    server.run().await
}
