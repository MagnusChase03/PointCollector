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

}
