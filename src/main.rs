mod agent;
mod env;

use agent::policy as p;
use agent::memory;
use env::maze;

use std::thread;
use rand::Rng;

fn do_move(policy: &mut p::Policy, replay: &mut memory::Memory, game: &mut maze::Maze, direction: char) -> Result<(), &'static str> {

    let start_state: Vec<f64> = vec![game.player_x as f64, game.player_y as f64, 
                                    game.goal_x as f64, game.goal_y as f64];

    match game.move_player(direction) {

        Ok(reward) => {

            replay.add(start_state,
                direction,
                reward);

            return Ok(());

        },
        Err(e) => Err::<(), &str>(e)

    };

    Ok(())

}

fn make_move(policy: &mut p::Policy, replay: &mut memory::Memory, game: &mut maze::Maze) -> Result<(), &'static str> {

    let inputs = vec![game.player_x as f64, game.player_y as f64, 
                    game.goal_x as f64, game.goal_y as f64];

    policy.forward(&inputs);
    

    let mut rng = rand::thread_rng();
    let num: f64 = rng.gen();
    let mut total: f64 = 0.0;
    for output in 0..policy.outputs.len() {

        total += policy.outputs[output];
        if num <= total {

            match output {

                0 => return do_move(policy, replay, game, 'u'),
                1 => return do_move(policy, replay, game, 'd'),
                2 => return do_move(policy, replay, game, 'l'),
                3 => return do_move(policy, replay, game, 'r'),
                _other => Err::<(), &str>("Incorrect number of outputs.")

            };

        }

    }

    Err("Incorrect number of outputs.")

}

fn main() {

    let mut policy = p::Policy::new(4, 4, 6);
    policy.randomize_weights();

    let mut replay = memory::Memory::new();

    // let test_inputs = vec![1.0, 2.0, 3.0, 4.0];
    // policy.forward(&test_inputs);

    // for i in 0..1000 {

    //     policy.backpropagate(&test_inputs, 10.0, 'u');

    // }

    let mut game = maze::Maze::new(10, 10);
    game.add_walls();

    make_move(&mut policy, &mut replay, &mut game);
    game.print();
    
    // for i in 0..30 {
        
    //     thread::sleep_ms(500);
    //     game.print();

    // }

    // println!("{:?}",policy);

}