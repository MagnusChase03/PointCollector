mod agent;

use agent::policy as p;

fn main() {

    let mut policy = p::Policy::new(4, 4, 6);
    policy.randomize_weights();

    let test_inputs = vec![1.0, 2.0, 3.0, 4.0];
    policy.forward(&test_inputs);

    for i in 0..1000 {

        policy.backpropagate(&test_inputs, 10.0, 'u');

    }

    // println!("{:?}",policy);

}