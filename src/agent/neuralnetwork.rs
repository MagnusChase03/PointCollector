use rand::Rng;

pub struct NeuralNetwork {

    weights: Vec<Vec<f64>>

}

impl NeuralNetwork {

    // Create a new neural network with random weights
    pub fn new() -> NeuralNetwork {

        let mut rng = rand::thread_rng();

        let mut weights: Vec<Vec<f64>> = Vec::new();
        
        let mut weight_set: Vec<f64> = Vec::new();
        for _i in 0..24 {

            weight_set.push(rng.gen());

        }

        weights.push(weight_set);
        
        let mut weight_set2: Vec<f64> = Vec::new();
        for _i in 0..24 {

            weight_set2.push(rng.gen());

        }

        weights.push(weight_set2);

        NeuralNetwork {weights: weights}

    }

}