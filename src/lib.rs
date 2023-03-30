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

#[wasm_bindgen]
pub fn lol() {
    spawn_local(async {
        const BECH32_SK: &str = "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85";
        let secret_key = SecretKey::from_bech32(BECH32_SK).unwrap();
        let keys = Keys::new(secret_key);
        let client = Client::new(&keys);
    
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.add_relay("wss://relay.rip").await.unwrap();
    
        client.connect().await;
    
        // Publish text note
        let event_id = client
            .publish_text_note("Testing nostr-sdk WASM", &[])
            .await
            .unwrap();
        // console::log_1(&format!("Event id: {event_id}").into());
    })
}
