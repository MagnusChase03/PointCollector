use rand::Rng;
use std::io::Write;
use std::io::Read;

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

    pub fn clone(&self) -> Policy {

        let mut copy_policy = Policy {

            inputs: vec![0.0; self.inputs.len()],

            hidden: vec![vec![0.0; self.hidden[0].len()]; 2],

            outputs: vec![0.0; self.outputs.len()],

            weights: vec![vec![vec![0.0; self.hidden[0].len()]; self.inputs.len()], 
                vec![vec![0.0; self.hidden[0].len()]; self.hidden[0].len()], 
                vec![vec![0.0; self.outputs.len()]; self.hidden[0].len()]],

            biases: vec![vec![0.0; self.hidden[0].len()], 
                vec![0.0; self.hidden[0].len()], 
                vec![0.0; self.outputs.len()]],

            learning_rate: 1.0,

            output_total: 0.0

        };

        for layer in 0..self.biases.len() {

            for node in 0..self.biases[layer].len() {

                copy_policy.biases[layer][node] = self.biases[layer][node];

            }

        }

        for layer in 0..self.weights.len() {

            for node in 0..self.weights[layer].len() {

                for weight in 0..self.weights[layer][node].len() {

                    copy_policy.weights[layer][node][weight] = self.weights[layer][node][weight];

                }

            }

        }

        copy_policy.learning_rate = self.learning_rate;

        copy_policy

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
    
    pub fn save_weights(&self, filepath: &str) {

        let mut file = std::fs::File::create(filepath).unwrap();
        for layer in 0..self.weights.len() {

            for node in 0..self.weights[layer].len() {

                for weight in 0..self.weights[layer][node].len() {

                    file.write_all(format!("{}\n", self.weights[layer][node][weight].to_string()).as_bytes()).unwrap();

                }   

            }   

        }

        for layer in 0..self.biases.len() {

            for node in 0..self.biases[layer].len() {

                file.write_all(format!("{}\n", self.biases[layer][node].to_string()).as_bytes()).unwrap();

            }

        }

    }

    pub fn load_weights(&mut self, filepath: &str) {

        let mut file = std::fs::File::open(filepath).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let split = contents.split("\n");
        let weights: Vec<&str> = split.collect();

        let mut num: usize = 0;
        for layer in 0..self.weights.len() {

            for node in 0..self.weights[layer].len() {

                for weight in 0..self.weights[layer][node].len() {

                    self.weights[layer][node][weight] = weights[num].parse::<f64>().unwrap();
                    num += 1;

                }   

            }   

        }

        for layer in 0..self.biases.len() {

            for node in 0..self.biases[layer].len() {

                self.biases[layer][node] = weights[num].parse::<f64>().unwrap();
                num += 1;

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

                    total += self.inputs[node] * self.weights[0][node][to];

                }

                return Ok(total + self.biases[0][to]);

            },
            1 => {

                for node in 0..self.hidden[0].len() {

                    total += self.hidden[0][node] * self.weights[1][node][to];

                }

                return Ok(total + self.biases[1][to]);

            },
            2 => {

                for node in 0..self.hidden[1].len() {

                    total += self.hidden[1][node] * self.weights[2][node][to];

                }

                return Ok(total + self.biases[2][to]);

            },
            _ => {
               
                return Err("Invalid layer to forward.");
                
            }

        }

    }

    pub fn forward(&mut self, inputs: &Vec<f64>) -> Result<(), &'static str> {

        for num in 0..inputs.len() {

            self.inputs[num] = inputs[num];

        }

        for node in 0..self.hidden[0].len() {

            match Self::forward_single(self, 0, node) {

                Ok(total) => {

                    self.hidden[0][node] = Self::sigmoid(total);

                },
                Err(e) => return Err(e),

            };

        }

        for node in 0..self.hidden[1].len() {

            match Self::forward_single(self, 1, node) {

                Ok(total) => {

                    self.hidden[1][node] = Self::sigmoid(total);

                },
                Err(e) => return Err(e),

            };

        }

        let mut output_total: f64 = 0.0;
        for output in 0..self.outputs.len() {

            match Self::forward_single(self, 2, output) {

                Ok(total) => {

                    self.outputs[output] = total;
                    output_total += total;

                },
                Err(e) => return Err(e),

            };

        }

        self.output_total = output_total;
        for output in 0..self.outputs.len() {

            self.outputs[output] = Self::liklyhood(self.outputs[output], output_total);

        }

        Ok(())

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

    pub fn backpropagate(&mut self, inputs: &Vec<f64>, reward: f64, action: char, copy_policy: &mut Policy) -> Result<(), &'static str> {

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

        copy_policy.forward(inputs);

        let mut error: f64 = 0.0;
        if reward > 0.0 {

            error = copy_policy.outputs[output_node] - 1.0;

        } else {

            error = copy_policy.outputs[output_node];

        }

        error = error * reward.abs();

        let derivatives: f64 = (2.0 * error * copy_policy.learning_rate) 
                / copy_policy.output_total;

        self.biases[2][output_node] -= derivatives;
        for node in 0..self.hidden[1].len() {

            self.weights[2][node][output_node] -= derivatives * copy_policy.hidden[1][node];

        }

        let mut hidden1_error: Vec<f64> = vec![0.0; self.hidden[1].len()];
        for node in 0..self.hidden[1].len() {

            match Self::forward_single(copy_policy, 1, node) {

                Ok(total) => {

                    hidden1_error[node] = derivatives * copy_policy.weights[2][node][output_node] 
                        * Self::sigmoid_d(total);

                    self.biases[1][node] -= hidden1_error[node];
                    for prev_node in 0..self.hidden[0].len() {

                        self.weights[1][prev_node][node] -= hidden1_error[node] * copy_policy.hidden[0][prev_node];

                    }

                },
                Err(e) => return Err(e),

            };

        }

        for node in 0..self.hidden[0].len() {

            for next_node in 0..self.hidden[1].len() {

                match Self::forward_single(copy_policy, 0, node) {

                    Ok(total) => {

                        let node_error: f64 = hidden1_error[next_node] * copy_policy.weights[1][node][next_node]
                            * Self::sigmoid_d(total);

                        self.biases[0][node] -= node_error;
                        for inp in 0..self.inputs.len() {

                            self.weights[0][inp][node] -= node_error * copy_policy.inputs[inp];

                        }

                    },
                    Err(e) => return Err(e),

                };

            }

        }

        Ok(())
        
    }

}