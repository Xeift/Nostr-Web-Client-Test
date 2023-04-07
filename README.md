# Nostr-Web-Client-Test
[繁體中文](https://github.com/Xeift/Nostr-Web-Client-Test/blob/main/README.md) | [English](https://github.com/Xeift/Nostr-Web-Client-Test/blob/main/README_en.md)

## 這是做什麼用的？
這個專案是 Nostr 的網頁版客戶端，使用 Rust WebAssembly 撰寫，仍在非常初期的開發階段。


## Nostr 簡介
Nostr 是一種去中心化的通訊協議，允許使用者連接到不同中繼器並發送訊息。

**中繼器**

就是伺服器，負責接收和轉發使用者傳送的訊息。

Nostr 只需要私鑰即可登入，沒有帳號密碼，並藉由公鑰密碼學驗證使用者的真實性。

## Rust 簡介

## WebAssembly 簡介

## 檔案架構
`src/lib.rs` 是使用 Rust 撰寫的 WebAssembly，這個檔案經過編譯後，結果會存在 `pkg` 資料夾。

`pkg` 中 `web_application.js` 會調用 `pkg/web_application.wasm`。

`website` 資料夾中存放的則是正常的 html css js，與傳統網站並無差異。

## 已實作功能
✅發送訊息

✅儲存私鑰至 local storge