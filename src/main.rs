use actix_web::{get, web, App, HttpServer, Responder};
use std::io;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> io::Result<()> {
    // Print a message indicating the server is starting
    println!("Starting the server at http://127.0.0.1:8080...");

    // Start the server
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    // Print a message indicating that the server has stopped
    println!("Server stopped.");
    Ok(())
}
