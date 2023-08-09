use actix_web::{get, App, HttpServer, Responder, HttpResponse};

const INDEX: &str = include_str!("../files/index.html");
const STATIC_STYLESHEET: &str = include_str!("../files/static/stylesheet.css");

#[get("/")]
async fn index() -> impl Responder {
    // PathBuf::from("files/index.html");
    // let path = PathBuf::from("files/index.html");
    // if let Ok(file) = NamedFile::open(path) {
    //     return file;
    // }
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX)
}

#[get("/static/stylesheet.css")]
async fn static_stylesheet() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(STATIC_STYLESHEET)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(static_stylesheet)
            // .service(actix_files::Files::new("/static", "files/static"))
    })
        .bind(("0.0.0.0", 80))?
        .run()
        .await
}
