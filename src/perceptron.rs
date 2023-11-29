mod perceptron {

use std::fs;

// model struct
struct Model {
    weights: Vec<f64>,
}

// data struct
struct Data {
    elements: Vec<Vec<f64>>,
    targets: Vec<i32>,
}

// populate data
impl Data {
    pub fn new(filename: str) -> Self {
        println!("========== LOADING DATA ==========");

        let contents = fs::read_to_string(filename)
            .expect("Failed to read file: {}.", filename);

        let lines = contents.split("\n")
            .collect();

        let mut elements: Vec<Vec<f64>> = Vec::new();
        let mut targets: Vec<i32> = Vec::new();

        for line in lines {
            let items = line.split(" ");
                .collect()
                .map(|s| s.parse<f64>().unwrap());

            targets.push(
                items.pop()
                   .unwrap()
                   .round() 
                   as i32
            );

            features.push(items);
        }

        println!("Successfully loaded data from file: {}!", filename);

        Data {
            elements,
            targets
        }
    }
}

// populate model
impl Model {
    pub fn new(data: &Data) -> Self {
        // get dimentions from data & set shape
        // create random array of weights
        // print successful Model init message
    }
}

// sgd

// fit model 

// predict

// evaluate model


}
