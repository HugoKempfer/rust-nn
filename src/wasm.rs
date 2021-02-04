use crate::Network;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_network(
    inputs_nb: usize,
    outputs_nb: usize,
    hiddens_nb: usize,
    learning_rate: f64,
) -> Network {
    Network::new(inputs_nb, outputs_nb, hiddens_nb, learning_rate)
}

#[wasm_bindgen]
pub fn damn() -> Result<i32, JsValue> {
    Ok(42)
}
