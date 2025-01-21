use wasm_bindgen::prelude::*;
use web_sys::console; // window

mod vm;
mod ql;
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
    headers: Vec<String>,
    raw_body: String,
}

#[wasm_bindgen]
impl RequestDetails {
    #[wasm_bindgen(getter)]
    pub fn get_ID(&self) -> String {
        self.ID.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_ID(&mut self, ID: String) {
        self.ID = ID;
    }
}

#[wasm_bindgen]
impl RequestDetails {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            ID: String::new(),
            target_url: String::new(),
            headers: vec![],
            raw_body: String::new(),
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_raw_body(&mut self, raw_body: String) {
        self.raw_body = raw_body;
    }

    #[wasm_bindgen(getter)]
    pub fn get_raw_body(&self) -> String {
        self.raw_body.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_target_url(&mut self, target_url: String) {
        self.target_url = target_url;
    }

    #[wasm_bindgen(getter)]
    pub fn get_target_url(&self) -> String {
        self.target_url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_headers(&mut self, headers: Vec<String>) {
        self.headers = headers;
    }

    #[wasm_bindgen(getter)]
    pub fn get_headers(&self) -> Vec<String> {
        self.headers.clone()
    }
}

#[wasm_bindgen]
pub fn save_incomming_request(req: RequestDetails) -> Result<(), String> {
    log!(
        "Request ID:{}, targetURL:{} , headers: {:?}, rawBody; {:?}",
        req.ID,
        req.target_url,
        req.headers,
        req.raw_body
    );
    Ok(())
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log!("Hello World!");
}
