use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn without_routing_macro() -> impl Responder {
    HttpResponse::Ok().body("Hello world, without routing macro!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/manual-route", web::get().to(without_routing_macro))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}