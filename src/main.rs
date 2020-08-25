use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("{\"status\":200}")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/ping", web::get().to(ping))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    println!("starting server...");
    server.run().await
}