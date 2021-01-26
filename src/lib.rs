use nalgebra::DMatrix;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::convert::TryInto;

pub struct Network {
    inputs_nb: i32,
    outputs_nb: i32,
    hiddens_nb: i32,
    learning_rate: f64,
    input_weights: DMatrix<f64>,
    hidden_weights: DMatrix<f64>,
}

fn init_layer_weight_values(layer_size: usize, previous_layer_size: i32) -> Vec<f64> {
    let mut rng = thread_rng();
    let prev_layer_sqrt = (previous_layer_size as f64).sqrt();
    let mut values = Vec::with_capacity(layer_size);
    let uni = Uniform::new(-1.0 / prev_layer_sqrt, 1.0 / prev_layer_sqrt);

    for _ in 0..layer_size {
        values.push(rng.sample(&uni));
    }
    values
}

impl Network {
    pub fn new(inputs_nb: i32, outputs_nb: i32, hiddens_nb: i32, learning_rate: f64) -> Self {
        Self {
            inputs_nb,
            outputs_nb,
            hiddens_nb,
            learning_rate,
            input_weights: DMatrix::from_vec(
                hiddens_nb.try_into().unwrap(),
                inputs_nb.try_into().unwrap(),
                init_layer_weight_values((inputs_nb * hiddens_nb) as usize, inputs_nb),
            ),
            hidden_weights: DMatrix::from_vec(
                outputs_nb.try_into().unwrap(),
                hiddens_nb.try_into().unwrap(),
                init_layer_weight_values((hiddens_nb * outputs_nb) as usize, hiddens_nb),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::init_layer_weight_values;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn weight_init() {
        let res = init_layer_weight_values(30, 20);
        assert_eq!(res.len(), 30);
    }
}
