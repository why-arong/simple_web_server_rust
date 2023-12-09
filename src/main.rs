use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::sync::Arc;
use nix::sys::signal::{self, SigSet, Signal, SigHandler, SigAction, SaFlags};
use nix::libc;
use std::thread;
use std::time::Duration;


static SIGHUP_RECEIVED: AtomicBool = AtomicBool::new(false);


extern "C" fn handle_sighup(_: libc::c_int) {
    SIGHUP_RECEIVED.store(true, Ordering::SeqCst);
    println!("SIGHUP signal received");
}

fn setup_signal_handler() {
    std::thread::spawn(move || {
        let sig_action = SigAction::new(
            SigHandler::Handler(handle_sighup),
            SaFlags::SA_RESTART,
            SigSet::empty(),
        );

        unsafe {
            signal::sigaction(Signal::SIGHUP, &sig_action).expect("Failed to set signal handler");
        }
        let mut sigset = SigSet::empty();
        sigset.add(Signal::SIGHUP);
    });
}

async fn greet(counter: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let count = counter.fetch_add(1, Ordering::SeqCst);

    if count >= 5 {
        // Return an HttpResponse for error case
        HttpResponse::BadRequest().body(format!("No more requests allowed {}", count))
    } else {
        // Convert the string response to an HttpResponse
        HttpResponse::Ok().body(format!("Hello! You are visitor number {}\n", count + 1))
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Arc::new(AtomicUsize::new(0));
    let pid = std::process::id();
    println!("The PID of this process is: {}", pid);
    setup_signal_handler();
    
    HttpServer::new(move || {
        let counter_clone = counter.clone();

        // Check the flag and reset the counter
        thread::spawn(move || {
            loop {
                if SIGHUP_RECEIVED.load(Ordering::SeqCst) {
                    counter_clone.store(0, Ordering::SeqCst);
                    SIGHUP_RECEIVED.store(false, Ordering::SeqCst);
                    println!("Counter reset after SIGHUP");
                }
                thread::sleep(Duration::from_millis(500));
            }
        });

        // HTTP server setup
        App::new()
            .app_data(web::Data::new(counter.clone()))
            .route("/", web::get().to(greet))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

