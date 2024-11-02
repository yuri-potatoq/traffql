use wasm_bindgen::prelude::*;
use web_sys::{console}; // window

// use js_sys::Promise;
// use std::time::Duration;
// use wasm_bindgen_futures::JsFuture;


// pub async fn sleep(duration: Duration) {
//     JsFuture::from(Promise::new(&mut |yes, _| {
//         window()
//             .unwrap()
//             .set_timeout_with_callback_and_timeout_and_arguments_0(
//                 &yes,
//                 duration.as_millis() as i32,
//             )
//             .unwrap();
//     })).await.unwrap();
// }

macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct RequestDetails {
    ID: String,
    target_url: String,
    headers: Vec<String>
}

#[wasm_bindgen]
pub fn save_incomming_request(req: RequestDetails) -> Result<(), String> {    
    log!("Request ID:{}, targetURL:{} , headers: {:?}", req.ID, req.target_url, req.headers);
    Ok(())
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log!("Hello World!");
}