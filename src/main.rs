mod handlers;
mod services;
mod models;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use rusqlite::Connection;
use std::sync::Mutex;
use std::fs as std_fs; // Alias the standard library's `fs` module
use tera::Tera;

use crate::handlers::index::{index, check_and_update_onion_status};

struct AppState {
    template_engine: Tera,
    db_connection: Mutex<Connection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Tera for templates
    let tera = Tera::new("templates/**/*").expect("Error loading templates");

    // Connect to SQLite database
    let conn = Connection::open("onions.db").expect("Failed to connect to database");

    // Shared application state
    let state = web::Data::new(AppState {
        template_engine: tera,
        db_connection: Mutex::new(conn),
    });

    // Read Tor hidden service hostname
    let tor_hostname_path = "/var/lib/tor/onionswatch/hostname";
    let tor_hostname = std_fs::read_to_string(tor_hostname_path) // Use the aliased `std_fs`
        .unwrap_or_else(|_| "Error: Unable to read Tor hostname".to_string());

    if tor_hostname.starts_with("Error") {
        eprintln!("{}", tor_hostname);
    } else {
        println!("Your Tor hidden service is accessible at: http://{}", tor_hostname.trim());
    }

    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .route("/check", web::get().to(check_and_update_onion_status)) // Optional route for status checking
            .service(fs::Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

