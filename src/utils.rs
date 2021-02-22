use std::fs::File;
use std::io::{ErrorKind, Read, Write};

use crate::Network;
use image::DynamicImage;
use nalgebra::DMatrix;

impl Network {
    pub fn save_to_file(&self, file_path: &str) -> Result<(), String> {
        if let Ok(serialized) = ron::to_string(&self) {
            match std::fs::write(file_path, serialized.as_bytes()) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("Cannot write on file: {}", err.to_string())),
            }
        } else {
            Err("Can't serialize the network.".to_string())
        }
    }

    pub fn read_from_file(filepath: &str) -> Result<Self, String> {
        if let Ok(mut file) = File::open(filepath) {
            let mut data = String::new();
            file.read_to_string(&mut data)
                .expect("Cannot read in file.");
            match ron::from_str(data.as_str()) {
                Ok(net) => Ok(net),
                Err(_) => Err("Cannot deserialize the network.".to_string()),
            }
        } else {
            Err(format!("Cannot open the file '{}'", filepath))
        }
    }
}
