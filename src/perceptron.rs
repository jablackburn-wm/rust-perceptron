mod perceptron {

use std::fs;
use rand::thread_rng;

// model struct
pub struct Model {
    data: &Data,
    weights: Vec<f64>,
}

// data struct
pub struct Data {
    elements: Vec<Vec<f64>>,
    targets: Vec<i32>,
}

// populate data
impl Data {
    pub fn new(filename: &str) -> Self {
        println!("========== LOADING DATA ==========");

        let contents = fs::read_to_string(filename)
            .expect("Failed to read data file");

        let lines = contents.split("\n")
            .collect();

        let mut elements: Vec<Vec<f64>> = Vec::new();
        let mut targets: Vec<i32> = Vec::new();

        for line in lines {
            let items = line.split(" ")
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

    pub fn display_data(&self) {
        let mut i = 0;
        for feature_vector in self.elements {
            print!("{:?}", feature_vector);
            println!("{}", self.targets[i]);
            i += 1;
        }
    }

}

impl Model {

    // populate model
    pub fn new(data: &Data) -> Self {
        println!("========== BUILDING MODEL ==========");

        // get dimentions from data & set shape
        let length = data.elements.len();

        // create random array of weights
        let weights: Vec<f64> = Vec::new();
        let mut range = rand::thread_rng();

        for i in 0..length {
            let weight = range.gen();
            weights.push( weight );
        }

        println!("Successfully initialized model");

        Model {
            data,
            weights
        }
    }

    // sgd
    fn sgd(&self, current_index: i32) {
        //TODO
    }

    // predict
    fn predict(&self) {
        //TODO
    }

    // fit model 
    fn fit(&self) {
        //TODO
    }

    // evaluate model
    fn evaluate(&self, filename: &str) {
        //TODO
    }

}

}
