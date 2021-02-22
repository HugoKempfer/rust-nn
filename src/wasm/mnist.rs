use crate::Network;
use image::imageops::FilterType;
use image::ImageFormat;
use nalgebra::DMatrix;
use std::panic;
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

fn get_boxed_result_from_matrix(matrix: DMatrix<f64>, output_nb: usize) -> Box<[f64]> {
    let mut outputs = Vec::with_capacity(output_nb);

    for idx in 0..output_nb {
        outputs.push(matrix[(idx, 0)]);
    }
    outputs.into_boxed_slice()
}

#[wasm_bindgen]
pub fn predict_for_mnist_dataset(net: &Network, image: Box<[f64]>) -> Result<Box<[f64]>, JsValue> {
    match net.predict(&image.to_vec()) {
        Ok(res) => return Ok(get_boxed_result_from_matrix(res, net.outputs_nb)),
        Err(err) => Err(JsValue::from(err)),
    }
}

#[wasm_bindgen]
pub fn predict_from_raw_canvas(net: &Network, base64_img: String) -> Result<Box<[f64]>, JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    match base64::decode(base64_img) {
        Ok(data) => {
            let img = image::load_from_memory_with_format(&*data, ImageFormat::Png)
                .expect("Cannot load image");
            let mut img = img.resize(28, 28, FilterType::Gaussian);
            img.invert();
            let gray_img = img.into_luma8();
            let img_bytes = gray_img.to_vec();
            if img_bytes.len() != 28 * 28 {
                return Err(JsValue::from_str("Wrong image size"));
            }
            let pixels: Vec<f64> = img_bytes
                .iter()
                .map(|pixel| (((*pixel as f64 / 255.0) * 0.99) + 0.01))
                .collect();
            match net.predict(&pixels) {
                Ok(res) => Ok(get_boxed_result_from_matrix(res, net.outputs_nb)),
                Err(err) => Err(JsValue::from(err)),
            }
        }
        Err(..) => Err(JsValue::from("Cannot decode Base64 image.")),
    }
}
