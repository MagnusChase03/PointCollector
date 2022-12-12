mod maze;

pub use crate::maze::grid;

fn main() {
    
    let mut maze = grid::Grid::new();
    maze.print();

    maze.move_player('D').unwrap();
    maze.print();

}
