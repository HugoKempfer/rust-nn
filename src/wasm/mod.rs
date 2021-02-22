use crate::Network;
use wasm_bindgen::prelude::*;

pub mod mnist;

#[wasm_bindgen]
impl Network {
    pub fn to_ron(&self) -> String {
        ron::to_string(self).expect("Cannot serialize to RON")
    }

    pub fn from_ron(net: String) -> Self {
        ron::from_str(&*net).expect("Cannot deserialize from RON")
    }
}
