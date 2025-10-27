# unitconv 🦀

**unitconv** adalah utilitas *command-line* ringan untuk konversi satuan, dibuat dengan Rust — cepat, sederhana, dan cocok dipakai langsung dari terminal.

[Repository](https://github.com/achmadichzan/unit-converter)

---

## ✨ Fitur Utama

* 🔁 **Konversi Suhu**: Celsius ⇄ Fahrenheit ⇄ Kelvin
* 📏 **Konversi Panjang**: cm ⇄ inch ⇄ km ⇄ miles
* ✅ **Validasi Cerdas**: menolak konversi antar kategori berbeda (mis. cm → celsius)
* 📜 **Riwayat Konversi**: menyimpan riwayat ke `conversion.json`
* 📚 **List Satuan**: `list` menampilkan semua satuan yang didukung
* ⚙️ Dibangun dengan: **Rust**, **clap** (CLI parsing), **serde** (JSON)

---

## 🚀 Quickstart — Jalankan dari source

Pastikan [Rust toolchain](https://rustup.rs/) sudah terinstall.

1. **Clone repo**

```bash
git clone https://github.com/achmadichzan/unit-converter.git
cd unitconv
```

2. **Jalankan dengan Cargo**

> gunakan `--` untuk memisahkan argumen Cargo dan argumen aplikasi

```bash
cargo run -- --help
```

Contoh output singkat:

```
Usage: unitconv.exe <COMMAND>

Commands:
  convert   Melakukan konversi dari satu satuan ke satuan lain
  list      Menampilkan daftar semua satuan yang didukung
  history   Menampilkan riwayat konversi
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Contoh-contoh pemakaian

**Konversi Suhu**

```bash
cargo run -- convert --from celsius --to fahrenheit --value 100
# Output: 100 °C = 212.0000 °F
```

**Konversi Panjang**

```bash
cargo run -- convert --from cm --to km --value 16000
# Output: 16000 cm = 0.1600 km
```

**Daftar Satuan**

```bash
cargo run -- list
# Menampilkan: celsius, fahrenheit, kelvin, cm, inch, km, miles
```

**Riwayat Konversi**

```bash
cargo run -- history
# Menampilkan riwayat yang tersimpan di conversion.json
```

**Contoh validasi**

```bash
cargo run -- convert --from cm --to celsius --value 87
# Output (error): tidak dapat mengonversi satuan yang berbeda kategori
```

---

## 📦 Build release (opsional)

Untuk membuat binary release yang teroptimasi:

```bash
cargo build --release
```

Binary terletak di `target/release/unitconv` (Windows: `unitconv.exe`).

Contoh menjalankan binary langsung:

```powershell
# PowerShell (Windows)
.\target\release\unitconv.exe convert --from kelvin --to celsius --value 273.15
# Output: 273.15 K = 0.0000 °C
```

---

## 🗂️ Lokasi riwayat

Riwayat konversi disimpan ke file `conversion.json` di direktori kerja aplikasi.
(Silakan tambah ke `.gitignore` agar tidak ikut ter-push.)

---

## 🛠️ Arsitektur singkat / implementasi

* CLI parsing: **clap**
* Serialisasi riwayat: **serde_json**
* Modular: modul terpisah untuk `units`, `parser`, `io`
* Error handling: hasilkan pesan user-friendly untuk input yang salah

---

## 🤝 Kontribusi

Senang menerima kontribusi — silakan:

1. Fork repository
2. Buat branch feature: `feat/my-feature`
3. Commit & push, lalu buka Pull Request

Tambahkan test sederhana untuk fungsi konversi agar integrasi mudah diverifikasi.

---

## 📬 Kontak

Jika ada fitur yang ingin ditambahkan atau bug, buka *issue* di repo:
[https://github.com/achmadichzan/unit-converter/issues](https://github.com/achmadichzan/unit-converter/issues)

---

Terima kasih sudah menggunakan **unitconv** — semoga membantu konversi cepat dari terminal! 🚀
