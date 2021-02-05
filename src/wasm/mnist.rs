use crate::Network;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn train_for_mnist_dataset(net: &mut Network, image: Box<[f64]>, labels: Box<[u8]>) {
    let mut targets: [f64; 10] = [0.1; 10];

    for (pos, el) in labels.iter().enumerate() {
        if *el == 1_u8 {
            targets[pos] = 0.99;
        }
    }
    net.train(&image.into_vec(), &targets.to_vec())
        .expect("Failed to train");
}

#[wasm_bindgen]
pub fn predict_for_mnist_dataset(net: &mut Network, image: Box<[f64]>) -> Box<[f64]> {
    let mut outputs = [0.0_f64; 10];

    match net.predict(&image.to_vec()) {
        Ok(res) => {
            for idx in 0..(net.outputs_nb) {
                outputs[idx] = res[(idx, 0)];
            }
        }
        Err(_) => eprintln!("Damn err"),
    }
    Box::new(outputs)
}
