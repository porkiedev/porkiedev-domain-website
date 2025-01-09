//
// Contains the code for the root domain
//

use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpServer, Responder};
use tracing::{info, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Serve the root path
#[get("/")]
async fn index() -> impl Responder {
    // Respond with the index file
    NamedFile::open_async("./assets/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a filter that limits the log level to TRACE for this module, and INFO for everything else
    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(Level::INFO)
        .with_target("module", Level::TRACE)
        .with_target(module_path!(), Level::TRACE);
    // Initialize the logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    // Initialize the server
    info!("Actix server is starting");
    HttpServer::new(|| {
        App::new()
            // Include the logger middleware
            .wrap(web::Logger)
            .service(index)
            .service(Files::new("/assets", "assets"))
    })
        .bind(("0.0.0.0", 80))?
        .run()
        .await
}
