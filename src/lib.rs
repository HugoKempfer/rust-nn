use nalgebra::{DMatrix, DimMul};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::convert::TryInto;

pub struct Network {
    inputs_nb: usize,
    outputs_nb: usize,
    hiddens_nb: usize,
    learning_rate: f64,
    input_weights: DMatrix<f64>,
    hidden_weights: DMatrix<f64>,
}

fn init_layer_weight_values(layer_size: usize, previous_layer_size: usize) -> Vec<f64> {
    let mut rng = thread_rng();
    let prev_layer_sqrt = (previous_layer_size as f64).sqrt();
    let mut values = Vec::with_capacity(layer_size);
    let uni = Uniform::new(-1.0 / prev_layer_sqrt, 1.0 / prev_layer_sqrt);

    for _ in 0..layer_size {
        values.push(rng.sample(&uni));
    }
    values
}

///https://en.wikipedia.org/wiki/Sigmoid_function
fn sigmoid(value: f64) -> f64 {
    1.0_f64 / (1.0_f64.powf(-1.0_f64 * value))
}

impl Network {
    pub fn new(inputs_nb: usize, outputs_nb: usize, hiddens_nb: usize, learning_rate: f64) -> Self {
        Self {
            inputs_nb,
            outputs_nb,
            hiddens_nb,
            learning_rate,
            input_weights: DMatrix::from_vec(
                hiddens_nb.try_into().unwrap(),
                inputs_nb.try_into().unwrap(),
                init_layer_weight_values(inputs_nb * hiddens_nb, inputs_nb),
            ),
            hidden_weights: DMatrix::from_vec(
                outputs_nb.try_into().unwrap(),
                hiddens_nb.try_into().unwrap(),
                init_layer_weight_values((hiddens_nb * outputs_nb) as usize, hiddens_nb),
            ),
        }
    }

    /// Proceed to one forward pass to predict the output values according to the
    /// model weights
    pub fn predict(&self, inputs: Vec<f64>) -> Result<DMatrix<f64>, String> {
        if inputs.len() != self.inputs_nb {
            return Err("Provided input size differs from model's one".to_string());
        }
        //Transpose the input values in an input matrix
        let input_matrix = DMatrix::from_vec(inputs.len(), 1, inputs);
        //Calculate the hidden layer input values from input weights and input values
        let mut hidden_inputs: DMatrix<f64> = &self.input_weights * &input_matrix;
        hidden_inputs.apply(sigmoid); // Apply activation function
        let mut hidden_outputs = &self.hidden_weights * &hidden_inputs;
        hidden_outputs.apply(sigmoid);
        Ok(hidden_outputs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{init_layer_weight_values, Network};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn weight_init() {
        let res = init_layer_weight_values(30, 20);
        assert_eq!(res.len(), 30);
    }

    #[test]
    fn create_network() {
        let net = Network::new(10, 10, 50, 0.5);
        assert_eq!(net.hidden_weights.len(), 50 * 10);
        assert_eq!(net.input_weights.len(), 10 * 50);
    }
}
