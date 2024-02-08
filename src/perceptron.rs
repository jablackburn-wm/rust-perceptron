pub mod perceptron {

use std::fs;
use rand::Rng;

// model struct
pub struct Model {
    data: Data,
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

        let lines = contents.lines();

        let mut elements: Vec<Vec<f64>> = Vec::new();
        let mut targets: Vec<i32> = Vec::new();

        for line in lines {
            let mut items = line.split(" ")
                .map(|s| s.parse::<f64>().unwrap())
                .collect::<Vec<f64>>();


            targets.push(
                items.pop()
                   .unwrap()
                   .round() 
                   as i32
            );

            elements.push(items);

        }

        println!("Successfully loaded data from file: {}! \n", filename);

        Data {
            elements,
            targets
        }
    }

    pub fn display_data(&self) {
        println!("========== DISPLAYING DATA ==========\n");

        let mut i = 0;
        for feature_vector in &self.elements {
            print!("{:?}", feature_vector);
            println!("  {}", self.targets[i]);
            i += 1;
        }
        println!("\n");
    }

}

impl Model {

    // populate model
    pub fn new(data: Data) -> Self {
        println!("========== BUILDING MODEL ==========");

        // get dimentions from data & set shape
        let length = data.elements[0].len() + 1;

        // create random array of weights
        let mut weights: Vec<f64> = Vec::new();
        let mut range = rand::thread_rng();

        for _i in 0..length {
            let weight = range.gen();
            weights.push( weight );
        }

        println!("Successfully initialized model\n");

        Model {
            data,
            weights
        }
    }

    pub fn display_model(&self) {
        println!("========== DISPLAYING MODEL WEIGHTS ==========\n");

        for weight in &self.weights {
            println!("{}", weight);
        }
        println!("\n");
    }

    // sgd
    fn sgd(&mut self, current_index: i32) {
        let target = self.data.targets[current_index];
        // get length from data struct
        // iterate backwards over data weights, weight += target * current feature
        // weights[0] += target
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
