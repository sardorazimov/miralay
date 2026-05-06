// miralay-cli/src/main.rs
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("🚀 Miralay göreve hazır! Proje paketleniyor...");
    
    // Basitlik adına projenin zip halini manuel hazırladığını varsayalım 
    // ya da rust-zip ile burayı otomatikleştiririz.
    let mut file = File::open("project.zip").expect("project.zip bulunamadı!");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Dosya okunamadı!");

    let client = Client::new();
    let res = client.post("http://miralay.miransas.com/deploy")
        .body(buffer)
        .header("X-Project-Name", "benim-web-sitem")
        .send();

    match res {
        Ok(_) => println!("✅ Miralay başarıyla mevzi aldı! Proje yayında."),
        Err(e) => println!("❌ Harekat başarısız: {}", e),
    }
}