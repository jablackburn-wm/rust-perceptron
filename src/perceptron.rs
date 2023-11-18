mod perceptron {

use std::fs;

// shape struct
struct Shape {
    rows: u32,
    columns: u32,
}

// model struct
struct Model {
    shape: Shape,
    weights: Vec<f64>,
}

// data struct
struct Data {
    shape: Shape,
    elements: Vec<Vec<f64>>,
    targets: Vec<f64>,
}

// get shape 

// populate data
impl Data {
    pub fn new(filename: str) -> Self {
        let contents = fs::read_to_string(filename)
            .expect("Failed to read file {}", filename);

        let lines = contents.split("\n")
            .collect();
        let rows = lines.len();

        // get lines, map to f64
        // collect into 2d Vector
        // print successful load message
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
