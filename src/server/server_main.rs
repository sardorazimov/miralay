// miralay-server/src/main.rs
use axum::{routing::post, Router, body::Bytes, http::HeaderMap};
use std::fs;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/deploy", post(handle_deploy));

    println!("🎖️ Miralay Sunucusu 3000 portunda nöbette...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_deploy(headers: HeaderMap, body: Bytes) {
    let project_name = headers.get("X-Project-Name").unwrap().to_str().unwrap();
    let path = format!("./deployments/{}", project_name);
    
    fs::create_dir_all(&path).unwrap();
    // Burada 'body' (zip verisi) ayıklanıp klasöre konulacak.
    println!("📦 '{}' projesi başarıyla konuşlandırıldı.", project_name);
}