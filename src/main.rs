mod maze;
mod agent;

pub use crate::maze::grid;
pub use crate::agent::neuralnetwork;

fn main() {
    
    let mut maze = grid::Grid::new();
    maze.print();

    maze.move_player('D').unwrap();
    maze.print();

    let mut network = neuralnetwork::NeuralNetwork::new();
    let test = network.forward(vec![maze.player.0, maze.player.1, maze.goal.0, maze.goal.1]);
    println!("{:?}", test);

}
