use clap::{App, Arg, Clap};
use mnist::{Mnist, MnistBuilder};
use pbr::ProgressBar;
use rust_nn::Network;

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
            current_offset += 28 * 28;
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
    let mut app = App::new("Rust NN for MNIST")
        .author("Hugo Kempfer <hugkempf@gmail.com>")
        .about("Demonstration of the multilayer perceptron usage for the MNIST handwritten digits dataset.")
        .arg(Arg::new("train").about("Train the model.").short('t').takes_value(false).required_unless_present("predict"))
        .arg(Arg::new("hidden").about("Number of hidden nodes to use").short('h').default_value("100").takes_value(true))
        .arg(Arg::new("rate").about("Learning rate value").short('r').default_value("0.1").takes_value(true))
        .arg(Arg::new("predict").about("Use a model to predict the test image set").short('p').takes_value(false).required_unless_present("train"))
        .arg(Arg::new("file").about("Model file to use (RON format)").short('f').default_missing_value("./model.ron").takes_value(true));
    let args = app.get_matches();
    let (trn_size, rows, cols) = (50_000_u32, 28, 28);

    let mut nn = if args.is_present("file") && !args.is_present("train") {
        match Network::read_from_file(args.value_of("file").unwrap()) {
            Ok(net) => net,
            Err(err) => {
                panic!("Error while opening model file: {}", err);
            }
        }
    } else {
        Network::new(
            rows * cols,
            10,
            args.value_of("hidden").unwrap().parse().unwrap(),
            args.value_of("rate")
                .unwrap()
                .parse()
                .expect("Invalid learning rate format"),
        )
    };
    let dataset = MnistBuilder::new()
        .download_and_extract()
        .label_format_digit()
        .training_set_length(trn_size)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .finalize();
    if args.is_present("train") || (args.is_present("predict") && !args.is_present("file")) {
        println!("Training a network with");
        train_dataset(&mut nn, &dataset);
        println!("Training done.");
    }
    if args.is_present("predict") {
        test_model(&nn, &dataset);
    }
    if args.is_present("file") && args.is_present("train") {
        let filename = args.value_of("file").unwrap();
        nn.save_to_file(filename).expect("Can't save to file");
        println!("Model saved under {}", filename);
    }
}
