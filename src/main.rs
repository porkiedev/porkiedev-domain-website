use actix_web::{get, App, HttpServer, Responder, HttpResponse, HttpRequest};
use log::{LevelFilter::Info, info};
use env_logger;

const INDEX: &str = include_str!("../files/index.html");
const STATIC_STYLESHEET: &str = include_str!("../files/static/stylesheet.css");

// Serve root
#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    // PathBuf::from("files/index.html");
    // let path = PathBuf::from("files/index.html");
    // if let Ok(file) = NamedFile::open(path) {
    //     return file;
    // }

    // Get HTTP request headers and create a new string for the logger
    let request_headers = req.headers();
    let mut log_string: String = String::new();

    // Try to resolve the client's IP address (Checks for a cloudflare proxy first, and then checking for direct client address)
    let client_ip_address = || {
        if let Some(client_ip) = request_headers.get("cf-connecting-ip") {
            return format!("{:?}", client_ip);
        }
        if let Some(client_ip) = req.peer_addr() {
            return format!("{}", client_ip);
        }
        return String::from("unknown-ip");
    };

    // Some pretty string formatting
    log_string.push_str(&format!("Incoming connection from '{}'\nRequest Headers:\n", client_ip_address()));

    for request_header in request_headers {
        log_string.push_str(&format!("  {}: {:?}\n", request_header.0, request_header.1));
    };

    // Log the HTTP request
    info!("{log_string}");

    // Respond with HTML
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX)
}

// Serve css stylesheet
#[get("/static/stylesheet.css")]
async fn static_stylesheet() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(STATIC_STYLESHEET)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::builder().filter_module(module_path!(), Info).init();

    // Initialize the server
    info!("Actix server is starting");
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
