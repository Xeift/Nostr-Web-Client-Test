# Nostr-Web-Client-Test
[繁體中文](https://github.com/Xeift/Nostr-Web-Client-Test/blob/main/README.md) | [English](https://github.com/Xeift/Nostr-Web-Client-Test/blob/main/README_en.md)

## 這是做什麼用的？
這個專案是 Nostr 的網頁版客戶端，使用 Rust WebAssembly 撰寫，仍在非常初期的開發階段。


## Nostr 簡介
Nostr 是一種去中心化的通訊協議，使用者可以自由連接到不同的中繼器並發送訊息。

Nostr 主要由中繼器與客戶端組成。

+ **中繼器**

    就是伺服器，負責接收和轉發使用者傳送的訊息。

    可以想成是玩手遊時要選的伺服器。

+ **客戶端**

    就是這個專案嘗試在打造的東西，包括使用者界面和各功能，主要用於和中繼器溝通，接收和傳送訊息。

    以手遊來比喻就是遊戲的 App。

+ **公鑰**

    識別你在 Nostr 上的身份。

    可以想成是門牌，只要知道門牌，就能找到你家。

    公鑰是公開的。

+ **私鑰**

    有私鑰 = 擁有該 Nostr 帳號的完整權限，可以發訊息、讀訊息、追蹤別人。

    可以想成是家裡鑰匙，有鑰匙就能進屋，不論要在廚房煎牛排或從保險箱搬金條出來都可以。

    私鑰應妥善保存，洩漏給他人知道會是場災難。

+ **簽章**

    Nostr 會藉由公鑰密碼學驗證使用者的真實性。

    使用者發訊息時會使用個人私鑰對內容做簽名。

    透過演算法的特性，任何人都可以用公鑰驗證 經私鑰簽名過的內容是否是本人所簽署。

## Rust 簡介

## WebAssembly 簡介

## 檔案架構
`src/lib.rs` 是使用 Rust 撰寫的 WebAssembly，這個檔案經過編譯後，結果會存在 `pkg` 資料夾。

`pkg` 中 `web_application.js` 會調用 `pkg/web_application.wasm`。

`website` 資料夾中存放的則是正常的 html css js，與傳統網站並無差異。

## 已實作功能
✅發送訊息

✅儲存私鑰至 local storge