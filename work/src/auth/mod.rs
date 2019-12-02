use actix_web::{get, App, HttpServer, Responder, web};



#[get("/{id}/{name}/index.html")]
fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

pub fn run() {
    println!("auth is running");

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap()
}