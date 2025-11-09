
use actix_web::{web, App, HttpServer, Responder};
use chrono::Utc;

async fn hello() -> impl Responder {
    web::Json(serde_json::json!({"message": "Hello from Rust!"}))
}

struct TimeResponse {
    current_time: String,
}

async fn time() -> Json<TimeResponse> {
    let now = Utc::now().to_rfc3339();
    Json(TimeResponse { current_time: now })
}

async fn fibonacci(info: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let n = info.get("n").and_then(|v| v.parse::<usize>().ok()).unwrap_or(10);
    let mut seq = Vec::new();
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        seq.push(a);
        let temp = a + b;
        a = b;
        b = temp;
    }
    web::Json(serde_json::json!({"sequence": seq}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(hello))
            .route("/fibonacci", web::get().to(fibonacci))
            .route("/time", web::get().to(time))
    })
    .bind(("0.0.0.0", 5003))?
    .run()
    .await
}
