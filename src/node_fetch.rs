///! Helper methods for using node-fetch.
use js_sys::{Array, Function, Promise};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn require(name: &str) -> Function;

    type Response;

    #[wasm_bindgen(method, js_name = json)]
    fn json(this: &Response) -> Promise;

    #[wasm_bindgen(method, js_name = text)]
    fn text(this: &Response) -> Promise;
}

/// Supported request methods.
pub enum FetchMethod {
    Get,
    Post,
    Put,
}

impl FetchMethod {
    /// Return the string representation of the fetch method.
    pub fn as_str(&self) -> &'static str {
        match self {
            &FetchMethod::Get => "GET",
            &FetchMethod::Post => "POST",
            &FetchMethod::Put => "PUT",
        }
    }
}

#[derive(Serialize)]
/// Options to configure a HTTP request.
struct RequestOptions {
    /// Request method: GET, PUT, POST, DELETE
    method: String,
    /// Body sent in request
    body: Option<String>,
    /// Request headers
    headers: HashMap<String, String>,
}

/// Perform a HTTP request with the provided options.
pub async fn fetch(
    url: &str,
    method: FetchMethod,
    body: &str,
    headers: HashMap<String, String>,
    empty_response: bool,
) -> Result<JsValue, JsValue> {
    // node-fetch needs to be installed
    let fetch = require("node-fetch");

    let body = match method {
        FetchMethod::Get => None, // Request with GET/HEAD method cannot have body
        _ => Some(body.to_owned()),
    };

    let options = RequestOptions {
        method: method.as_str().to_owned(),
        body: body,
        headers,
    };

    let fetch_result = fetch.apply(
        &JsValue::null(),
        &Array::of2(&JsValue::from(url), &JsValue::from_serde(&options).unwrap()),
    );
    match fetch_result {
        Ok(p) => {
            let promise = Promise::from(p);
            let resp_value = JsFuture::from(promise).await?;
            let resp: Response = resp_value.unchecked_into();

            if empty_response {
                Ok(JsValue::NULL)
            } else {
                let json: JsValue = JsFuture::from(resp.json()).await?;
                Ok(json)
            }
        }
        Err(e) => {
            console::log_1(&format!("Error executing fetch request {:?}", e).into());
            Err(JsValue::from(format!(
                "Error executing fetch request {:?}",
                e
            )))
        }
    }
}
