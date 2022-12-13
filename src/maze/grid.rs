use std::fmt;
use rand::Rng;

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
        Self::move_goal(&mut maze);

        maze

    }

    // Print maze for debugging
    pub fn print(&self) {

        for i in 0..self.grid.len() {

            println!("{:?}", self.grid[i]);

        }
        println!("");

    }

    // Randomly move the goal for after collection
    fn move_goal(maze: &mut Grid) {

        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(1..10);
        let mut y = rng.gen_range(1..10);
        while maze.grid[y][x] != Tile::Empty {

            x = rng.gen_range(1..10);
            y = rng.gen_range(1..10);

        }

        maze.grid[maze.goal.0][maze.goal.1] = Tile::Empty;
        maze.goal = (x, y);
        maze.grid[maze.goal.0][maze.goal.1] = Tile::Goal;

    }

    // Moves player in direction if available
    // TODO MAKE A FUNCTION FOR THE DIRECTIONS, THIS IS ABSURD
    pub fn move_player(&mut self, direction: char) -> i64 {

        match direction {

            'U' => {

                if self.grid[self.player.0 - 1][self.player.1] == Tile::Empty
                    || self.grid[self.player.0 - 1][self.player.1] == Tile::Goal {

                    let mut reward: i64 = -1;
                    if self.grid[self.player.0 - 1][self.player.1] == Tile::Goal {

                        Self::move_goal(self);
                        reward = 10;

                    }

                    self.grid[self.player.0][self.player.1] = Tile::Empty;

                    self.player = (self.player.0 - 1, self.player.1);
                    self.grid[self.player.0][self.player.1] = Tile::Player;
                    
                    return reward;

                } else {

                    return -1;

                }

            },
            'D' => {

                if self.grid[self.player.0 + 1][self.player.1] == Tile::Empty 
                    || self.grid[self.player.0 + 1][self.player.1] == Tile::Goal {

                    let mut reward: i64 = -1;
                    if self.grid[self.player.0 + 1][self.player.1] == Tile::Goal {

                        Self::move_goal(self);
                        reward = 10;

                    }

                    self.grid[self.player.0][self.player.1] = Tile::Empty;

                    self.player = (self.player.0 + 1, self.player.1);
                    self.grid[self.player.0][self.player.1] = Tile::Player;
                    
                    return reward;

                } else {

                    return -1;

                }

            },
            'L' => {

                if self.grid[self.player.0][self.player.1 - 1] == Tile::Empty 
                    || self.grid[self.player.0][self.player.1 - 1] == Tile::Goal {

                    let mut reward: i64 = -1;
                    if self.grid[self.player.0][self.player.1 - 1] == Tile::Goal {

                        Self::move_goal(self);
                        reward = 10;

                    }

                    self.grid[self.player.0][self.player.1] = Tile::Empty;

                    self.player = (self.player.0, self.player.1 - 1);
                    self.grid[self.player.0][self.player.1] = Tile::Player;
                    
                    return reward;

                } else {

                    return -1;

                }

            },
            'R' => {

                if self.grid[self.player.0][self.player.1 + 1] == Tile::Empty 
                    || self.grid[self.player.0][self.player.1 + 1] == Tile::Goal {

                    let mut reward: i64 = -1;
                    if self.grid[self.player.0][self.player.1 + 1] == Tile::Goal {

                        Self::move_goal(self);
                        reward = 10;

                    }

                    self.grid[self.player.0][self.player.1] = Tile::Empty;

                    self.player = (self.player.0, self.player.1 + 1);
                    self.grid[self.player.0][self.player.1] = Tile::Player;
                    
                    return reward;

                } else {

                    return -1;

                }

            },
            _other => {

                println!("Invalid direction");
                return 0;

            }

        }

    }

}