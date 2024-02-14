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
    rows: usize,
    columns: usize,
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

        let rows = elements.len();
        let columns = elements[0].len() + 1;

        println!("Successfully loaded data from file: {}! \n", filename);

        Data {
            elements,
            targets,
            rows,
            columns
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

        let length = data.columns;

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
    fn sgd(&mut self, current_index: usize) {
        let target = self.data.targets[current_index];

        for i in 1..self.data.columns {
            self.weights[i] += target as f64 * self.data.elements[current_index][self.data.columns - i];
        }

        self.weights[0] += target as f64;
    }

    // predict
    fn predict(&self, current_index: usize) -> isize {
        let mut hypothesis: f64 = 0.0;

        for i in 0..self.data.columns {
            hypothesis += self.weights[self.data.columns - i] * self.data.elements[current_index][i];
        }

        if hypothesis < 0.0 { return -1; }
        1
    }

    // fit model 
    fn fit(&mut self) {
        let mut misclassified = true;
        while (misclassified) {
            misclassified = false;
            for i in 0..self.data.rows {
                let hypothesis = self.predict(i);
                let target = self.data.targets[i];
                // TODO if target == hypothesis continue
                self.sgd(i);
                misclassified = true;
            }
        }
    }

    // evaluate model
    fn evaluate(&self, filename: &str) {
        //TODO
    }

}

}
