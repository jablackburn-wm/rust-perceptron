mod perceptron {

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

    }
}

// populate model
impl Model {
    pub fn new(data: &Data) -> Self {

    }
}

// fit model 

// sgd

// predict

// evaluate model


}
