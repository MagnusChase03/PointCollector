use rand::Rng;

pub struct NeuralNetwork {

    pub weights: Vec<Vec<f64>>

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

    // Activation funtion
    fn relu(x: f64) -> f64 {

        if x < 0.0 {

            return 0.0

        }

        x

    }

    // Forward propagate the currecnt state to predict next action to take
    // TODO Dont hard code loops, maybe change in the future
    pub fn forward(&self, state: &Vec<usize>) -> Vec<f64> {

        let mut hidden_layer: Vec<f64> = vec![0.0;6];

        for i in 0..6 {

            for s in 0..state.len() {

                hidden_layer[i] += (state[s] as f64) * self.weights[0][4 * i + s];

            }

            hidden_layer[i] = Self::relu(hidden_layer[i]);

        }

        let mut output_layer: Vec<f64> = vec![0.0;4];
        let mut total: f64 = 0.0;
        for i in 0..4 {

            for h in 0..hidden_layer.len() {

                output_layer[i] += hidden_layer[h] * self.weights[1][6 * i + h];

            }

            output_layer[i] = Self::relu(output_layer[i]);
            total += output_layer[i];

        }

        for i in 0..4 {

            output_layer[i] = output_layer[i] / total;

        }

        output_layer

    }

    pub fn update(&mut self, state: Vec<usize>, direction: char, reward: i64) {



    }

}