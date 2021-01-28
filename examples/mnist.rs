use mnist::{Mnist, MnistBuilder, NormalizedMnist};
use pbr::{MultiBar, ProgressBar};
use rust_nn::Network;
use std::convert::TryInto;

fn train_dataset(net: &mut Network, dataset: &Mnist) {
    let (trn_size, rows, cols) = (50_000_u32, 28, 28);
    let mut progress_bar = ProgressBar::new((trn_size * 5) as u64);

    let mut inputs: Vec<f64> = Vec::with_capacity(rows * cols);
    let mut targets: Vec<f64> = Vec::with_capacity(10);

    let mut progress_bar_stepper = 0; // refresh progress every x iter
    for _ in 0..5 {
        let mut current_offset: usize = 0;
        for label in &dataset.trn_lbl {
            targets.clear();
            for idx in 0..10 {
                if *label as i32 == idx {
                    targets.push(0.99);
                } else {
                    targets.push(0.01);
                }
            }
            inputs.clear();
            let slice_end = current_offset + (28 * 28);
            for pixel in &dataset.trn_img[current_offset..slice_end] {
                inputs.push((((*pixel as f64 / 255.0) * 0.99) + 0.01) as f64);
            }
            net.train(&inputs, &targets).expect("damn");
            current_offset += (28 * 28);
            if progress_bar_stepper == 1000 {
                progress_bar.add(1000);
                progress_bar_stepper = 0;
            } else {
                progress_bar_stepper += 1;
            }
        }
    }
}

fn test_model(nn: &Network, dataset: &Mnist) {
    let (trn_size, rows, cols) = (50_000_u32, 28, 28);
    let mut score = 0;
    let mut inputs: Vec<f64> = Vec::with_capacity(rows * cols);
    let mut current_offset: usize = 0;

    for label in &dataset.tst_lbl {
        inputs.clear();
        let slice_end = current_offset + (28 * 28);
        for pixel in &dataset.tst_img[current_offset..slice_end] {
            inputs.push((((*pixel as f64 / 255.0) * 0.99) + 0.01) as f64);
        }
        match nn.predict(&inputs) {
            Ok(res) => {
                let mut best = 0;
                let mut highest = 0.0;
                for idx in 0..(nn.outputs_nb) {
                    let current = res[(idx, 0)];
                    if current > highest {
                        best = idx;
                        highest = current;
                    }
                }
                if best == (*label).into() {
                    score += 1;
                }
                println!("Current {}, predicted {}", label, best);
            }
            Err(_) => eprintln!("Damn err"),
        }
        current_offset += 28 * 28;
    }
    println!("Score => {}", score);
}

fn main() {
    let (trn_size, rows, cols) = (50_000_u32, 28, 28);
    let dataset = MnistBuilder::new()
        .download_and_extract()
        .label_format_digit()
        .training_set_length(trn_size)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .finalize();
    let mut nn = Network::new(28 * 28, 10, 100, 0.1);
    //let mut nn = Network::read_from_file(&"./model.ron".to_string()).unwrap();
    train_dataset(&mut nn, &dataset);
    test_model(&nn, &dataset);
    nn.save_to_file(&"model.ron".to_string())
        .expect("Can't save to file")
}
