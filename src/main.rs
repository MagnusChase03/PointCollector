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

fn train(policy: &mut p::Policy, replay: &agent::memory::Memory) -> Result<(), &'static str> {

    let mut value: f64 = 0.0;
    for mem in 0..replay.start_states.len() {

        let index: usize = replay.start_states.len() - mem - 1;
        if replay.rewards[index] > 0.0 {

            value = replay.rewards[index];

        } else {

            value = replay.rewards[index] + (0.75 * value);

        }

        match policy.backpropagate(&replay.start_states[index], value, replay.actions[index]) {

            Ok(()) => {},
            Err(e) => return Err(e),

        };

    }

    Ok(())

}

fn workout(policy: &mut p::Policy, sets: usize) {
    
    let mut rng = rand::thread_rng();

    for set in 0..sets {

        let mut explore_rate: f64 = 0.8;
        if set == 2 {

            explore_rate = 0.6;

        } else if set >= 4 {

            explore_rate = 0.4;

        }
        
        let games_to_play: i64 = 5000;
        let num_of_rounds: i64 = 120;
        let mut replays = vec![memory::Memory::new(); 100];
        for game_num in 0..games_to_play {
            
            println!("Game {}", game_num);
            let mut game = maze::Maze::new(10, 10);
            game.add_walls();

            for _round in 0..num_of_rounds {
            

                let num: f64 = rng.gen();
                if num < 0.1 {

                    match make_move(policy, &mut replays[(game_num % 100) as usize], &mut game, true) {

                        Ok(()) => {},
                        Err(e) => println!("{}", e), 

                    };

                } else {

                    match make_move(policy, &mut replays[(game_num % 100) as usize], &mut game, false) {

                        Ok(()) => {},
                        Err(e) => println!("{}", e),

                    };

                }

            }

            
            if game_num % 100 == 0 {
                
                for replay in 0..replays.len() {

                    match train(policy, &replays[replay]) {

                        Ok(()) => {},
                        Err(e) => println!("{}", e),

                    };
                    replays[replay] = memory::Memory::new();

                }
                // explore_rate -= 0.6 / (games_to_play as f64 / 100.0);
                explore_rate -= 0.2 / (games_to_play as f64 / 100.0);
            }

        }

        policy.save_weights(&format!("sets/{}.dat", set));

    }

}

fn play(policy: &mut p::Policy, num_of_rounds: usize) {

    let mut replays = vec![memory::Memory::new(); 1];   
    let mut game = maze::Maze::new(10, 10);
    game.add_walls();
    game.print();

    
    for _round in 0..num_of_rounds {
        
        thread::sleep_ms(20);
        match make_move(policy, &mut replays[0], &mut game, false) {

            Ok(()) => {},
            Err(e) => println!("{}", e),

        };

        game.print();

    }

}

fn main() {

    let mut policy = p::Policy::new(4, 4, 6);
    policy.randomize_weights();
    // policy.load_weights("sets/6.dat");
    policy.learning_rate = 0.00000001;

    workout(&mut policy, 10);
    play(&mut policy, 120);

}