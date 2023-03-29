use wasm_bindgen::prelude::*;


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