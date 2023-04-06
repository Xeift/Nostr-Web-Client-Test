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


async fn lol2(input: String) {
    // const BECH32_SK: &str = "nsec1lyc8k4krke9s72zatj8nckx99w6348kd7zc9n7k8dszvg86dp48s3gp8fy";
    let BECH32_SK: &str = &input[..];
    let secret_key = SecretKey::from_bech32(BECH32_SK).unwrap();
    let keys = Keys::new(secret_key);
    let client = Client::new(&keys);
    client.add_relay("wss://relay.snort.social").await.unwrap();
    client.connect().await;

    let event_id = client
        .publish_text_note("Testing nostr-sdk WASM", &[])
        .await
        .unwrap();
}


#[wasm_bindgen]
pub fn lol(input: &str) {
    let input = input.clone().to_string();
    spawn_local(lol2(input));
}
