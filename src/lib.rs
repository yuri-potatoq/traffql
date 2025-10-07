use js_sys::{Array, Promise, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::console; // window

pub mod ql;


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

#[derive(Debug)]
struct HeaderString(pub Vec<String>);

impl HeaderString {
    fn decode(&self) -> Option<Vec<(String, String)>> {
        Some(
            self.0
                .iter()
                .fold(Vec::<(String, String)>::new(), |mut acc, next| {
                    let mut splited = next.split(":");
                    let key = splited.next().unwrap_or("");
                    let value = splited.next().unwrap_or("");

                    acc.push((key.trim().to_string(), value.trim().to_string()));
                    acc
                }),
        )
    }
}

type Headers = Vec<String>;

#[wasm_bindgen]
#[derive(Debug)]
pub struct RequestDetails {
    ID: String,
    target_url: String,
    headers: Headers,
    raw_body: String,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ResponseDetails {
    request_id: String,
    headers: Headers,
    raw_body: String,
}

#[wasm_bindgen]
impl RequestDetails {
    #[wasm_bindgen(constructor)]
    pub fn new(ID: String, target_url: String, raw_body: String, headers: Headers) -> Self {
        Self {
            ID,
            target_url,
            raw_body,
            headers,
        }
    }
}

#[wasm_bindgen]
impl ResponseDetails {
    #[wasm_bindgen(constructor)]
    pub fn new(request_id: String, raw_body: String, headers: Headers) -> Self {
        Self {
            request_id,
            raw_body,
            headers,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub type SQLStorage;

    #[wasm_bindgen(structural, method)]
    pub fn query(this: &SQLStorage, query: String) -> Promise;
}

#[wasm_bindgen]
pub struct QueryEngine {
    storage: SQLStorage,
}

#[wasm_bindgen]
impl QueryEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(storage: SQLStorage) -> Self {
        Self { storage }
    }

    // #[wasm_bindgen(catch)]
    // pub fn catch() -> Result<(), JsValue> {
    //     Ok(())
    // }

    #[wasm_bindgen]
    pub async fn execute_query(&self, ql_expr: String) -> Result<RequestDetails, String> {
        log!("[WASM-LIB][execute_query] {ql_expr}");

        // parse query into SQL
        // execute query plan
        let promise = self.storage.query(format!("select * from request"));
        let result = JsFuture::from(promise).await.unwrap();

        let array = Array::from(&result);

        for i in 0..array.length() {
            let row = array.get(i);

            // Access another field
            let url_field = JsValue::from_str("url");
            if let Ok(url) = Reflect::get(&row, &url_field) {
                if let Some(url_str) = url.as_string() {
                    log!("[parsed value] url: {}", url_str);
                }
            }
        }

        // parse results
        // return data

        // what type should be the data?
        Ok(RequestDetails {
            ID: String::new(),
            headers: Headers::new(),
            raw_body: String::new(),
            target_url: String::new(),
        })
    }
}

// http.headers.has_key("Authorization") AND http.status_code == 200

#[wasm_bindgen]
pub fn save_request_data(req: RequestDetails) {
    log!(
        "[WASM-LIB][save_request_data] Request ID :{}, targetURL:{} , headers: {:?}, rawBody; {:?}",
        req.ID,
        req.target_url,
        HeaderString(req.headers).decode(),
        req.raw_body
    );
}

#[wasm_bindgen]
pub fn save_response_data(resp: ResponseDetails) {
    log!(
        "[WASM-LIB][save_response_data] Request ID :{}, headers: {:?}, rawBody; {:?}",
        resp.request_id,
        resp.raw_body,
        HeaderString(resp.headers).decode(),
    );
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log!("[WASM-LIB] Starting WASM!");
}

#[cfg(test)]
mod test {
    use crate::HeaderString;

    #[test]
    fn test_single_header_string() {
        let headers = vec![format!("Content-Type: application/json")];

        let result = HeaderString(headers).decode();

        assert_eq!(
            result,
            Some(vec![(format!("Content-Type"), format!("application/json"))])
        )
    }
}
