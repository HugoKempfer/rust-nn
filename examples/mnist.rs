use mnist::{Mnist, MnistBuilder};
use rust_nn::Network;
use std::convert::TryInto;

fn train_dataset(net: &mut Network) {
    let (trn_size, rows, cols) = (50_000, 28, 28);

    // Deconstruct the returned Mnist struct.
    let dataset = MnistBuilder::new()
        .download_and_extract()
        .label_format_digit()
        .training_set_length(trn_size)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .finalize()
        .normalize();

    let mut inputs: Vec<f64> = Vec::with_capacity(rows * cols);
    let mut targets: Vec<f64> = Vec::with_capacity(10);

    for epoch in 0..5 {
        let mut current_offset: usize = 0;
        for label in &dataset.trn_lbl {
            targets.clear();
            for idx in 0..10 {
                if *label as i32 == idx {
                    targets.push(*label as f64);
                } else {
                    targets.push(0.1);
                }
            }
            inputs.clear();
            for pixel in &dataset.trn_img[current_offset..current_offset + (28 * 28)] {
                inputs.push(*pixel as f64);
            }
            net.train(&inputs, &targets).expect("damn");
            current_offset += 28;
            println!("Current offset {}", current_offset);
        }
    }
}

fn main() {
    let mut nn = Network::new(28 * 28, 10, 100, 0.1);
    train_dataset(&mut nn);
    nn.save_to_file(&"damn".to_string());
}
