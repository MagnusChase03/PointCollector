use std::fmt;
use rand::Rng;
use std::thread;

#[derive(Clone, PartialEq)]
pub enum Tile {

    Empty,
    Wall,
    Player,
    Goal

}

impl fmt::Debug for Tile {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {

            Tile::Empty => write!(f, " "),
            Tile::Wall => write!(f, "#"),
            Tile::Player => write!(f, "P"),
            Tile::Goal => write!(f, "G"),

        }

    }

}

pub struct Maze {

    pub x_size: usize,
    pub y_size: usize,

    pub player_x: usize,
    pub player_y: usize,

    pub goal_x: usize,
    pub goal_y: usize,

    pub board: Vec<Vec<Tile>>

}

impl Maze {

    pub fn new(x_size: usize, y_size: usize) -> Maze {

        let mut the_maze = Maze {x_size: x_size, y_size: y_size, 
                                player_x: 1, player_y: 1,
                                goal_x: x_size - 2, goal_y: y_size - 2,
                                board: vec![vec![Tile::Empty; x_size]; y_size]};

        the_maze.add_walls();
        the_maze.board[1][1] = Tile::Player;
        the_maze.board[y_size - 2][x_size - 2] = Tile::Goal;

        the_maze

    }

    pub fn add_walls(&mut self) {

        for col in 0..self.board[0].len() {

            self.board[0][col] = Tile::Wall;
            self.board[self.y_size - 1][col] = Tile::Wall;

        }

        for row in 0..self.board.len() {

            self.board[row][0] = Tile::Wall;
            self.board[row][self.x_size - 1] = Tile::Wall;

        }

    }

    fn move_goal(&mut self) {

        let mut rng = rand::thread_rng();
        let mut x: usize = rng.gen_range(1..(self.x_size - 2));
        let mut y: usize = rng.gen_range(1..(self.y_size - 2));

        while (!(self.board[y][x] == Tile::Empty)) {

            x = rng.gen_range(1..self.x_size - 2);
            y = rng.gen_range(1..self.y_size - 2);

        }

        self.board[self.goal_y][self.goal_x] = Tile::Empty;
        self.goal_x = x;
        self.goal_y = y;
        self.board[self.goal_y][self.goal_x] = Tile::Goal;

    }

    fn move_player_dir(&mut self, x: i8, y: i8) -> Result<f64, &'static str> {

        let mut reward: f64 = -1.0;
        if x < 0 || y < 0 {

            if self.board[self.player_y - (y.abs() as usize)][self.player_x - (x.abs() as usize)] == Tile::Empty 
                || self.board[self.player_y - (y.abs() as usize)][self.player_x - (x.abs() as usize)] == Tile::Goal {

                if self.board[self.player_y - (y.abs() as usize)][self.player_x - (x.abs() as usize)] == Tile::Goal {

                    Self::move_goal(self);
                    println!("Goal!");
                    reward = 120.0;

                }

                self.board[self.player_y][self.player_x] = Tile::Empty;
                self.player_y -= y.abs() as usize;
                self.player_x -= x.abs() as usize;
                self.board[self.player_y][self.player_x] = Tile::Player;

                return Ok(reward);
            } 

            return Ok(reward);

        } else if x > 0 || y > 0 {

            if self.board[self.player_y + (y as usize)][self.player_x + (x as usize)] == Tile::Empty
                || self.board[self.player_y + (y as usize)][self.player_x + (x as usize)] == Tile::Goal {


                if self.board[self.player_y + (y as usize)][self.player_x + (x as usize)] == Tile::Goal {

                    Self::move_goal(self);
                    println!("Goal!");
                    reward = 120.0;

                }

                self.board[self.player_y][self.player_x] = Tile::Empty;
                self.player_y += y as usize;
                self.player_x += x as usize;
                self.board[self.player_y][self.player_x] = Tile::Player;

                return Ok(reward);
            } 

            return Ok(reward);

        }

        Err("Both x and y are zero")

    }

    pub fn move_player(&mut self, direction: char) -> Result<f64, &'static str> {

        match direction {

            'u' => {

                return Self::move_player_dir(self, 0, -1);

            },
            'd' => {

                return Self::move_player_dir(self, 0, 1);

            },
            'l' => {

                return Self::move_player_dir(self, -1, 0);

            },
            'r' => {

                return Self::move_player_dir(self, 1, 0);

            },
            _other => Err::<f64, &'static str>("Invalid direction")

        }

    }

    pub fn print(&self) {

        for row in 0..self.board.len() {

            for col in 0..self.board[row].len() {

                print!("{:?} ", self.board[row][col]);

            }
            println!("");

        }

    }

}