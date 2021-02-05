use std::convert::TryInto;

use nalgebra::DMatrix;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Network {
    inputs_nb: usize,
    pub outputs_nb: usize,
    hiddens_nb: usize,
    learning_rate: f64,
    hidden_weight: DMatrix<f64>,
    output_weights: DMatrix<f64>,
}

pub mod utils;
pub mod wasm;

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
    1.0_f64 / (1.0 + std::f64::consts::E.powf(-1.0_f64 * value))
}

fn sigmoid_derivative(weights: &DMatrix<f64>) -> DMatrix<f64> {
    let one_matrix: DMatrix<f64> =
        DMatrix::from_vec(weights.nrows(), 1, vec![1.0; weights.nrows()]);

    weights.component_mul(&(one_matrix - weights))
}

#[wasm_bindgen]
impl Network {
    #[wasm_bindgen(constructor)]
    pub fn new(inputs_nb: usize, outputs_nb: usize, hiddens_nb: usize, learning_rate: f64) -> Self {
        Self {
            inputs_nb,
            outputs_nb,
            hiddens_nb,
            learning_rate,
            hidden_weight: DMatrix::from_vec(
                hiddens_nb.try_into().unwrap(),
                inputs_nb.try_into().unwrap(),
                init_layer_weight_values(inputs_nb * hiddens_nb, inputs_nb),
            ),
            output_weights: DMatrix::from_vec(
                outputs_nb.try_into().unwrap(),
                hiddens_nb.try_into().unwrap(),
                init_layer_weight_values((hiddens_nb * outputs_nb) as usize, hiddens_nb),
            ),
        }
    }
}

impl Network {
    /// Immutably forward propagate and return output matrix
    fn forward_propagate(&self, inputs: &DMatrix<f64>) -> DMatrix<f64> {
        //Calculate the hidden layer input values from input weights and input values
        let mut hidden_inputs: DMatrix<f64> = &self.hidden_weight * inputs;
        hidden_inputs.apply(sigmoid); // Apply activation function
        let mut hidden_outputs = &self.output_weights * &hidden_inputs;
        hidden_outputs.apply(sigmoid);
        hidden_outputs
    }

    /// Proceed to one forward pass to predict the output values according to the
    /// model weights
    pub fn predict(&self, inputs: &Vec<f64>) -> Result<DMatrix<f64>, String> {
        //Transpose the input values in an input matrix
        let input_matrix = DMatrix::from_vec(inputs.len(), 1, (&inputs).to_vec());
        Ok(self.forward_propagate(&input_matrix))
    }

    pub fn train(&mut self, inputs: &Vec<f64>, targets: &Vec<f64>) -> Result<(), String> {
        if inputs.len() != self.inputs_nb {
            return Err("Provided input size differs from model's one".to_string());
        }
        //Transpose the input values in an input matrix
        let wow = [0.1_f64; 10];
        let input_matrix = DMatrix::from_vec(inputs.len(), 1, inputs.to_vec());
        let target_matrix = DMatrix::from_vec(targets.len(), 1, targets.to_vec());

        //Calculate the hidden layer input values from input weights and input values
        let mut hidden_inputs: DMatrix<f64> = &self.hidden_weight * &input_matrix;
        hidden_inputs.apply(sigmoid); // Apply activation function
        let mut hidden_outputs = &self.output_weights * &hidden_inputs;
        hidden_outputs.apply(sigmoid);

        let output_error: DMatrix<f64> = &target_matrix - &hidden_outputs;
        let hidden_error = &self.output_weights.transpose() * &output_error;

        self.output_weights = self.output_weights.clone()
            + (output_error.component_mul(&sigmoid_derivative(&hidden_outputs))
                * &hidden_inputs.transpose())
                .scale(self.learning_rate);

        self.hidden_weight = self.hidden_weight.clone()
            + ((hidden_error.component_mul(&sigmoid_derivative(&hidden_inputs))
                * input_matrix.transpose())
            .scale(self.learning_rate));
        Ok(())
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
        assert_eq!(net.output_weights.len(), 50 * 10);
        assert_eq!(net.hidden_weight.len(), 10 * 50);
    }
}
