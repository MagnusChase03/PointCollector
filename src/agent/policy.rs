use rand::Rng;

#[derive(Debug)]
pub struct Policy {

    pub inputs: Vec<f64>,
    pub hidden: Vec<Vec<f64>>,
    pub outputs: Vec<f64>,

    pub weights: Vec<Vec<Vec<f64>>>,
    pub biases: Vec<Vec<f64>>,

    pub learning_rate: f64,
    pub output_total: f64

}

impl Policy {

    pub fn new(inputs: usize, outputs: usize, hidden_size: usize) -> Policy {

        Policy {

            inputs: vec![0.0; inputs],

            hidden: vec![vec![0.0; hidden_size]; 2],

            outputs: vec![0.0; outputs],

            weights: vec![vec![vec![0.0; hidden_size]; inputs], 
                vec![vec![0.0; hidden_size]; hidden_size], 
                vec![vec![0.0; outputs]; hidden_size]],

            biases: vec![vec![0.0; hidden_size], 
                vec![0.0; hidden_size], 
                vec![0.0; outputs]],

            learning_rate: 1.0,

            output_total: 0.0

        }

    }

    pub fn randomize_weights(&mut self) {

        let mut rng = rand::thread_rng();

        for layer in 0..self.weights.len() {

            for node in 0..self.weights[layer].len() {

                for weight in 0..self.weights[layer][node].len() {

                    self.weights[layer][node][weight] = rng.gen();

                }

            }

        }

    }

    fn sigmoid(x: f64) -> f64 {

        1.0 / (1.0 + (-x).exp())

    }

    fn sigmoid_d(x: f64) -> f64 {

        Self::sigmoid(x) * (1.0 - Self::sigmoid(x))

    }


    fn liklyhood(x: f64, total: f64) -> f64 {

        x / total

    }

    fn forward_single(&self, layer: i64, to: usize) -> Result<f64, &'static str> {

        let mut total: f64 = 0.0;
        match layer {

            0 => {

                for node in 0..self.inputs.len() {

                    total += self.inputs[node] * self.weights[0][node][to] + self.biases[0][to];

                }

                return Ok(total);

            },
            1 => {

                for node in 0..self.hidden[0].len() {

                    total += self.hidden[0][node] * self.weights[1][node][to] + self.biases[1][to];

                }

                return Ok(total);

            },
            2 => {

                for node in 0..self.hidden[1].len() {

                    total += self.hidden[1][node] * self.weights[2][node][to] + self.biases[2][to];

                }

                return Ok(total);

            },
            _ => {
               
                return Err("Invalid layer to forward.");
                
            }

        }

    }

    pub fn forward(&mut self, inputs: &Vec<f64>) {

        for num in 0..inputs.len() {

            self.inputs[num] = inputs[num];

        }

        for node in 0..self.hidden[0].len() {

            // let mut total: f64 = 0.0;
            // for num in 0..self.inputs.len() {

            //     total += self.inputs[num] * self.weights[0][num][node] + self.biases[0][node];

            // }

            self.hidden[0][node] = Self::sigmoid(Self::forward_single(self, 0, node).unwrap());

        }

        for node in 0..self.hidden[1].len() {

            // let mut total: f64 = 0.0;
            // for node in 0..self.hidden[0].len() {

            //     total += self.hidden[0][node] * self.weights[1][node][node2] + self.biases[1][node2];

            // }

            self.hidden[1][node] = Self::sigmoid(Self::forward_single(self, 1, node).unwrap());

        }

        let mut output_total: f64 = 0.0;
        for output in 0..self.outputs.len() {

            // let mut total: f64 = 0.0;
            // for node in 0..self.hidden[1].len() {

            //     total += self.hidden[1][node] * self.weights[2][node][output] + self.biases[2][output];

            // }

            let total: f64 = Self::forward_single(self, 2, output).unwrap();
            self.outputs[output] = total;
            output_total += total;

        }

        self.output_total = output_total;
        for output in 0..self.outputs.len() {

            self.outputs[output] = Self::liklyhood(self.outputs[output], output_total);

        }

    }

    fn action_to_output(action: char) -> Result<usize, &'static str> {

        match action {

            'u' => Ok(0),
            'd' => Ok(1),
            'l' => Ok(2),
            'r' => Ok(3),
            _ => Err("Not a valid action.")

        }

    }

    pub fn backpropagate(&mut self, inputs: &Vec<f64>, reward: f64, action: char) -> Result<(), &'static str> {

        if reward == 0.0 {

            return Ok(());

        }

        let convert_action = Self::action_to_output(action);
        let mut output_node: usize = 0;

        match convert_action {

            Ok(node) => output_node = node,
            Err(e) => {

                return Err(e);

            }

        }

        self.forward(inputs);

        let mut error: f64 = 0.0;
        if reward > 0.0 {

            error = self.outputs[output_node] - 1.0;

        } else {

            error = self.outputs[output_node];

        }

        error = error * reward;

        let derivatives: f64 = (2.0 * error * Self::sigmoid_d(Self::forward_single(self, 2, output_node).unwrap()) * self.learning_rate) 
                / self.output_total;

        self.biases[2][output_node] -= derivatives;
        for node in 0..self.hidden[1].len() {

            self.weights[2][node][output_node] -= derivatives * self.hidden[1][node];

        }

        // for node in 0..self.hidden[1].len() {

        //     let node_error = derivatives * self.weights[2][node][output_node] 
        //         * Self::sigmoid_d(Self::forward_single(self, 1, node).unwrap());

        //     self.biases[1][node] -= node_error;
        //     for prev_node in 0..self.hidden[0].len() {

        //         self.weights[1][prev_node][node] -= node_error * self.hidden[0][prev_node];

        //     }

        // }

        let mut hidden1_error: Vec<f64> = vec![0.0; self.hidden[1].len()];
        for node in 0..self.hidden[1].len() {

            hidden1_error[node] = derivatives * self.weights[2][node][output_node] 
                * Self::sigmoid_d(Self::forward_single(self, 1, node).unwrap());

            self.biases[1][node] -= hidden1_error[node];
            for prev_node in 0..self.hidden[0].len() {

                self.weights[1][prev_node][node] -= hidden1_error[node] * self.hidden[0][prev_node];

            }

        }

        // for node in 0..self.hidden[0].len() {

        //     for node2 in 0..self.hidden[1].len() {

        //         self.weights[1][node][node2] -= hidden1_error[node] * self.hidden[0][node];

        //     }

        // }

        for node in 0..self.hidden[0].len() {

            // hidden0_error[node] = derivatives * self.weights[2][node][output_node] 
            //     * Self::sigmoid_d(Self::forward_single(self, 1, node).unwrap());

            for next_node in 0..self.hidden[1].len() {

                let node_error: f64 = hidden1_error[next_node] * self.weights[1][node][next_node]
                    * Self::sigmoid_d(Self::forward_single(self, 0, node).unwrap());

                self.biases[0][node] -= node_error;
                for inp in 0..self.inputs.len() {

                    self.weights[0][inp][node] -= node_error * self.inputs[inp];

                }

            }

        }

        Ok(())
        

    }

}