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

fn make_move(policy: &mut p::Policy, replay: &mut memory::Memory, game: &mut maze::Maze, explore: bool) -> Result<(), &'static str> {

    let inputs = vec![game.player_x as f64, game.player_y as f64, 
                    game.goal_x as f64, game.goal_y as f64];

    policy.forward(&inputs);
    

    let mut rng = rand::thread_rng();
    let num: f64 = rng.gen();
    let mut total: f64 = 0.0;
    let chance: f64 = 1.0 / policy.outputs.len() as f64;

    for output in 0..policy.outputs.len() {

        if explore {

            total += chance;

        } else {

            total += policy.outputs[output];

        }
        
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

fn train(policy: &mut p::Policy, replay: &agent::memory::Memory) {

    let mut value: f64 = 0.0;
    for mem in 0..replay.start_states.len() {

        let index: usize = replay.start_states.len() - mem - 1;
        if replay.rewards[index] > 0.0 {

            value = replay.rewards[index];

        } else {

            value += replay.rewards[index];

        }

        policy.backpropagate(&replay.start_states[index], value, replay.actions[index]);

    }

}

fn main() {

    let mut rng = rand::thread_rng();

    let mut policy = p::Policy::new(4, 4, 6);
    policy.randomize_weights();

    let mut explore_rate: f64 = 1.0;

    // let test_inputs = vec![1.0, 2.0, 3.0, 4.0];
    // policy.forward(&test_inputs);

    // for i in 0..1000 {

    //     policy.backpropagate(&test_inputs, 10.0, 'u');

    // }

    
    
    for game_num in 0..10000 {
        
        println!("Game {}", game_num);
        let mut game = maze::Maze::new(10, 10);
        game.add_walls();
        // game.print();

        let mut replay = memory::Memory::new();

        for _round in 0..60 {
        

            let num: f64 = rng.gen();
            if num < explore_rate {

                make_move(&mut policy, &mut replay, &mut game, true);

            } else {

                make_move(&mut policy, &mut replay, &mut game, false);

            }

            // game.print();

        }

        train(&mut policy, &replay);
        if game_num % 100 == 0 && explore_rate > 0.2 {

            explore_rate -= 0.016;

        }

    }

    let mut game = maze::Maze::new(10, 10);
    game.add_walls();
    game.print();

    let mut replay = memory::Memory::new();

    for _round in 0..60 {
    
        thread::sleep_ms(200);
        make_move(&mut policy, &mut replay, &mut game, false);

        game.print();

    }

    // println!("{:?}",policy);

}