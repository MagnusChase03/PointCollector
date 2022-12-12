use std::fmt;

// Enum to describe what each tile in the maze is
#[derive(Clone, PartialEq)]
pub enum Tile {

    Empty,
    Wall,
    Player,
    Goal

}

// Custom debug formating
impl fmt::Debug for Tile {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self {

            Tile::Empty => write!(f, "-"),
            Tile::Wall => write!(f, "#"),
            Tile::Player => write!(f, "P"),
            Tile::Goal => write!(f, "G"),

        }

    }

}

// Struct for the maze
pub struct Grid {

    pub grid: Vec<Vec<Tile>>,
    pub player: (usize, usize),
    pub goal: (usize, usize)

}

impl Grid {

    // Create a default "maze"
    pub fn new() -> Grid {

        let mut maze = Grid {grid: vec![vec![Tile::Empty; 12]; 12], player: (1, 1), goal: (10, 10)};
        for i in 0..maze.grid.len() {

            maze.grid[0][i] = Tile::Wall;
            maze.grid[11][i] = Tile::Wall;
            maze.grid[i][0] = Tile::Wall;
            maze.grid[i][11] = Tile::Wall;

        }
        maze.grid[1][1] = Tile::Player;
        maze.grid[10][10] = Tile::Goal;

        maze

    }

    // Print maze for debugging
    pub fn print(&self) {

        for i in 0..self.grid.len() {

            println!("{:?}", self.grid[i]);

        }
        println!("");

    }

    // Moves player in direction if available
    pub fn move_player(&mut self, direction: char) -> Result<(), &str> {

        match direction {

            'U' => {

                return Ok(());

            },
            'D' => {

                if self.grid[self.player.0 + 1][self.player.1] == Tile::Empty {

                    self.grid[self.player.0][self.player.1] = Tile::Empty;

                    self.player = (self.player.0 + 1, self.player.1);
                    self.grid[self.player.0][self.player.1] = Tile::Player;
                    
                    return Ok(());

                } else {

                    return Err("Cannot move in that direction.");

                }

            },
            'L' => {

                return Ok(());

            },
            'R' => {

                return Ok(());

            },
            _ => Err("Did not provide a correct direction.")

        }

    }

}