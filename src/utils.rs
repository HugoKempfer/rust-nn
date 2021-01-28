use std::fs::File;
use std::io::{ErrorKind, Read, Write};

use crate::Network;

impl Network {
    pub fn save_to_file(&self, file_path: &String) -> Result<(), String> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => match File::create(file_path) {
                    Ok(file) => file,
                    Err(err) => return Err(err.to_string()),
                },
                _ => return Err(err.to_string()),
            },
        };
        if let Ok(serialized) = ron::to_string(&self) {
            match file.write_all(serialized.as_bytes()) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("Cannot write on file: {}", err.to_string())),
            }
        } else {
            Err("Can't serialize the network.".to_string())
        }
    }

    pub fn read_from_file(filepath: &String) -> Result<Self, String> {
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
