mod maze;
mod agent;

pub use crate::maze::grid;
pub use crate::agent::neuralnetwork;

use rand::Rng;

// Exploits the currecnt state using the policy network
// TODO Clean this up a bit, and change for when move player return a reward
fn exploit(maze: &mut grid::Grid, network: &neuralnetwork::NeuralNetwork) {

    let guess = network.forward(vec![maze.player.0, maze.player.1, maze.goal.0, maze.goal.1]);

    let mut rng = rand::thread_rng();
    let rand_value: f64 = rng.gen();
    let mut total: f64 = 0.0;
    for i in 0..guess.len() {

        total += guess[i];
        if rand_value < total {

            match i {

                0 => {

                    match maze.move_player('U') {

                        Ok(reward) => (),
                        Err(e) => println!("{}", e)

                    }

                },
                1 => {

                    match maze.move_player('D') {

                        Ok(reward) => (),
                        Err(e) => println!("{}", e)

                    }

                },
                2 => {

                    match maze.move_player('L') {

                        Ok(reward) => (),
                        Err(e) => println!("{}", e)

                    }

                },
                3 => {

                    match maze.move_player('R') {

                        Ok(reward) => (),
                        Err(e) => println!("{}", e)

                    }

                },
                _other => println!("Output was not expected size.")

            }
            break;

        }

    }

}

fn main() {
    
    let mut maze = grid::Grid::new();
    maze.print();

    let mut network = neuralnetwork::NeuralNetwork::new();
    
    for i in 0..10 {

        exploit(&mut maze, &network);
    
        maze.print();

    }

}
