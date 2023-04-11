use chrono::Local;
use nostr_sdk::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap();
    log::debug!("hello world");
}


#[wasm_bindgen]
pub fn greet() -> i32 {
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // panic!("fweefwef");
    return 35235;
}


async fn lol2(privatekey: String, message: String) {
    let bech_32: &str = &privatekey[..];
    let message_new: &str = &message[..];
    let secret_key = SecretKey::from_bech32(bech_32).unwrap();
    let keys = Keys::new(secret_key);
    let client = Client::new(&keys);
    client.add_relay("wss://relay.snort.social").await.unwrap();
    client.connect().await;

    let current_time = Local::now().to_string();
    let content = format!("Test {} {}",message_new, current_time);
    let _event_id = client
        .publish_text_note(content, &[])
        .await
        .unwrap();
}


#[wasm_bindgen]
pub fn send_message(privatekey: &str, message: &str) {
    let privatekey = privatekey.clone().to_string();
    let message = message.clone().to_string();

    spawn_local(lol2(privatekey, message));
}
