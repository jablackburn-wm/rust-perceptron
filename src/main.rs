mod perceptron;
use crate::perceptron::perceptron::*;

fn main() {
    let data = Data::new("train.dat");
    data.display_data();

    let mut model = Model::new(data);
    model.display_model();
    model.fit();
    model.display_model();

    model.evaluate("test.dat");
}

