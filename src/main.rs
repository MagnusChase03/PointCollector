mod maze;
mod agent;

pub use crate::maze::grid;
pub use crate::agent::neuralnetwork;
pub use crate::agent::memory;

use rand::Rng;

// Exploits the currecnt state using the policy network
// TODO Save to replay memory
fn exploit(maze: &mut grid::Grid, network: &neuralnetwork::NeuralNetwork, replay_mem: &mut memory::ReplayMemory) {

    let start_state = vec![maze.player.0, maze.player.1, maze.goal.0, maze.goal.1];
    let guess = network.forward(&start_state);

    let mut rng = rand::thread_rng();
    let rand_value: f64 = rng.gen();
    let mut total: f64 = 0.0;
    for i in 0..guess.len() {

        total += guess[i];
        if rand_value < total {

            let mut direction: char = 'U';
            let mut reward: i64 = 0;
            match i {

                0 => {

                    direction = 'U';
                    reward = maze.move_player('U');
                    

                },
                1 => {

                    direction = 'D';
                    reward = maze.move_player('D');

                },
                2 => {

                    direction = 'L';
                    reward = maze.move_player('L');

                },
                3 => {

                    direction = 'R';
                    reward = maze.move_player('R');

                },
                _other => println!("Output was not expected size.")

            }
            replay_mem.add_memory(start_state, vec![maze.player.0, maze.player.1, maze.goal.0, maze.goal.1], direction, reward);
            break;

        }

    }

}

fn main() {
    
    let mut maze = grid::Grid::new();
    // maze.print();

    let mut network = neuralnetwork::NeuralNetwork::new();
    let mut replay_mem = memory::ReplayMemory::new();
    
    for i in 0..30 {

        exploit(&mut maze, &network, &mut replay_mem);
    
        // maze.print();

    }

    // println!("{:?}", replay_mem);

}
