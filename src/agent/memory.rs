#[derive(Debug)]
pub struct ReplayMemory {

    pub inital_states: Vec<Vec<usize>>,
    pub final_states: Vec<Vec<usize>>,
    pub directions: Vec<char>,
    pub rewards: Vec<i64>

}

impl ReplayMemory {

    pub fn new() -> ReplayMemory {

        ReplayMemory {inital_states: Vec::new(), final_states: Vec::new(), directions: Vec::new(), rewards: Vec::new()}

    }

    pub fn add_memory(&mut self, start_state: Vec<usize>, end_state: Vec<usize>, direction: char, reward: i64) {

        self.inital_states.push(start_state);
        self.final_states.push(end_state);
        self.directions.push(direction);
        self.rewards.push(reward);

    }

}