mod perceptron {

// shape struct
struct shape {
    rows: u32,
    columns: u32,
}

// model struct
struct model {
    shape: struct shape,
    weights: Vec<f64>,
}

// data struct
struct data {
    shape: struct shape,
    elements: Vec<Vec<f64>>,
    targets: Vec<f64>,
}

// get shape 

// populate data
impl data {
    pub fn new(filename: str) -> Self {

    }
}

// populate model
impl model {
    pub fn new(data: data) -> Self {

    }
}

// fit model 

// sgd

// predict

// evaluate model


}
