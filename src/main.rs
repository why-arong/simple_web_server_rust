use actix_web::{web, App, HttpServer, Responder};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

async fn greet(counter: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let count = counter.fetch_add(1, Ordering::SeqCst);

    if count >= 5 {
        // Logic to stop the server
        println!("Received 5 GET requests, shutting down.");
        std::process::exit(0);
    }

    format!("Hello! You are visitor number {}", count + 1)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pid = std::process::id();
    println!("The PID of this process is: {}", pid);
    let counter = Arc::new(AtomicUsize::new(0));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(counter.clone()))
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
