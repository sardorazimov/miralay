# 🎖️ Miralay

**Miralay**, Rust ile geliştirilmiş, Vercel benzeri hafif, hızlı ve güvenli bir deployment otomasyon aracıdır. "Miransas" ekosisteminin bir parçası olarak, projelerinizi saniyeler içinde uzak sunuculara konuşlandırmanızı sağlar.

> **Motto:** No Talking, Write Code, Deploy Fast.

## 🚀 Özellikler

-   **Hızlı Harekat:** Rust'ın gücüyle paketleme ve gönderim işlemleri milisaniyeler içinde tamamlanır.
-   **Hafif Sıklet:** Gereksiz bağımlılıklar içermez, sadece ihtiyacınız olanı sunar.
-   **Güçlü Komuta:** CLI üzerinden tek komutla tüm projeyi sunucuya fırlatır.
-   **Güvenli Mevzi:** Statik dosyalarınızı izole edilmiş klasörlerde barındırır.

## 🛠️ Mimari Yapı

Miralay iki ana bileşenden oluşur:

1.  **Miralay CLI (`miralay-cli`):** Yerel makinenizde çalışan, dosyaları paketleyip sunucuya gönderen birim.
2.  **Miralay Server (`miralay-server`):** Sunucu tarafında nöbet tutan, gelen paketleri karşılayıp yayına alan birim.

## 📦 Kurulum ve Kullanım

Projeyi yerelinizde derlemek için:

```bash
git clone [https://github.com/hesap-adi/miralay.git](https://github.com/hesap-adi/miralay.git)
cd miralay
cargo build --release