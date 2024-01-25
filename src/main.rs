mod perceptron;
use perceptron::*;

fn main() {
    let data = Data::new("train.dat");
    data::display_data();
}

