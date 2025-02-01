#![cfg(target_arch = "wasm32")]

use rune::runtime;
use wasm_bindgen::prelude::*;
use web_sys::{window, Blob, Response, Url};
use js_sys::{Promise, Uint8Array};

#[wasm_bindgen]
pub async fn run(entrypoint_url: String) {
    let response = fetch(&url).await?;
    let binary = fs::read(&input_path).expect("Failed to read the WASM file");
    runtime::run(input_path, binary);
}

async fn fetch(url: &str) -> Result<Response, JsValue> {
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::Cors);

    let request = web_sys::Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into()?;
    Ok(resp)
}
