use crate::Network;
use wasm_bindgen::prelude::*;

pub mod mnist;

#[wasm_bindgen]
pub fn damn() -> Result<i32, JsValue> {
    Ok(42)
}
