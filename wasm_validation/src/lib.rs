use shared::CreateHostParams;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_create_host_params(s: JsValue) -> JsValue {
    let as_str: String = serde_wasm_bindgen::from_value(s).unwrap();
    let res = CreateHostParams::parse_and_validate(&as_str);
    serde_wasm_bindgen::to_value(&res).unwrap()
}
