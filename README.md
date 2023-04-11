# Nostr-Web-Client-Test
[繁體中文](https://github.com/Xeift/Nostr-Web-Client-Test/blob/main/README.md) | [English](https://github.com/Xeift/Nostr-Web-Client-Test/blob/main/README_en.md)

## 這是做什麼用的？
這個專案是 Nostr 的網頁版客戶端，使用 Rust WebAssembly 撰寫，仍在非常初期的開發階段。

![image](https://user-images.githubusercontent.com/80938768/230636194-f94440fb-3769-4e63-8c60-0706e29e6eb7.png)


## Nostr 簡介
Nostr 是一種去中心化的通訊協議，使用者可以自由連接到不同的中繼器並發送訊息。

Nostr 主要由中繼器與客戶端組成。

+ **中繼器**

    就是伺服器，負責接收和轉發使用者傳送的訊息。

    可以想成是玩手遊時要選的伺服器。

    任何人都可以自由運行中繼器，使用者也可自由選擇要連到哪些中繼器。

+ **客戶端**

    就是這個專案正在嘗試打造的東西，包括使用者界面和各功能，主要用於和中繼器溝通，接收和傳送訊息。

    以手遊來比喻就是遊戲的 App。

    開發者可以開發各種客戶端，使用者可以自由選擇要使用的客戶端。

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
Rust 是一種非常安全且高效的高階語言，效能甚至等效 C++。

## WebAssembly 簡介
WebAssembly 目前沒有比較好的中文翻譯，雖然組合語言的英文是 Assembly，但 WebAssembly 與 Assembly 並不是相同的東西。

WebAssembly 簡單來說是用 JavaScript 以外的程式語言（大多是C、C++、Rust 等高執行效率的語言）寫邏輯的部分，經過編譯後的函數可以在網頁中與 JavaScript 搭配使用，大幅提升運行效率。

## 檔案架構
`src/lib.rs` 是使用 Rust 撰寫的 WebAssembly，這個檔案經過編譯後，結果會存在 `pkg` 資料夾。

`pkg` 中 `web_application.js` 會調用 `pkg/web_application.wasm`。

`website` 資料夾中存放的則是正常的 html css js，與傳統網站並無差異。

## 已實作功能
✅發送自訂訊息

✅儲存私鑰至 local storge

## 待完成
☐ 產生私鑰

☐ 驗證私鑰的格式是否正確

☐ 若私鑰已存在 local storge，則顯示編輯框

☐ 讀取訊息

## 筆記
`wasm-pack build --target web`

編譯 Rust 至 WASM