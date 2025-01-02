use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use rusqlite::{Connection, Result};
use serde::Serialize;
use tera::Context;
use crate::services::db::{get_onions, update_onion_status};
use crate::services::proxy::check_url_via_socks_proxy;

#[derive(Serialize)]
struct Onion {
    name: String,
    address: String,
    status: String,
    status_nb: String,
}

fn fetch_onions_from_db(conn: &Connection) -> Result<Vec<Onion>> {
    let mut stmt = conn.prepare("SELECT name, address, status, status_nb FROM onions")?;
    let rows = stmt.query_map([], |row| {
        Ok(Onion {
            name: row.get(0)?,
            address: row.get(1)?,
            status: row.get(2)?,
            status_nb: row.get(3)?,
        })
    })?;

    let mut onions = Vec::new();
    for onion in rows {
        onions.push(onion?);
    }
    Ok(onions)
}

pub async fn index(data: web::Data<crate::AppState>) -> impl Responder {
    let conn = data.db_connection.lock().unwrap();
    let onions = match fetch_onions_from_db(&conn) {
        Ok(onions) => onions,
        Err(err) => {
            eprintln!("Failed to fetch onions from DB: {}", err);
            return HttpResponse::InternalServerError().body("Error fetching data");
        }
    };

    let mut context = Context::new();
    context.insert("onions", &onions);

    // Properly format the UTC time and append " UTC"
    let utc_now = Utc::now();
    let formatted_time = format!("{}", utc_now.format("%a, %d %b %Y %H:%M:%S UTC"));
    context.insert("last_update", &formatted_time);

    let rendered = match data.template_engine.render("index.html", &context) {
        Ok(html) => html,
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            return HttpResponse::InternalServerError().body("Error rendering page");
        }
    };

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn check_and_update_onion_status(data: web::Data<crate::AppState>) -> HttpResponse {
    let proxy_address = "socks5h://127.0.0.1:9050"; // Adjust if necessary

    {
        let conn = data.db_connection.lock().unwrap();
        let onions = get_onions(&conn);

        for onion in onions {
            let status = match check_url_via_socks_proxy(&onion.address, proxy_address).await {
                Ok(status) => status,
                Err(_) => "offline".to_string(),
            };
            update_onion_status(&conn, onion.id, &status);
        }
    }

    HttpResponse::Ok().body("Onion statuses updated.")
}
