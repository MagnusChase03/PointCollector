pub struct Memory {

    pub start_states: Vec<Vec<f64>>,
    pub actions: Vec<char>,
    pub rewards: Vec<f64>

}

impl Memory {

    pub fn new() -> Memory {

        Memory {start_states: Vec::new(),
                actions: Vec::new(),
                rewards: Vec::new()}

    }

    pub fn add(&mut self, start_state: Vec<f64>, action: char, reward: f64) {

        self.start_states.push(start_state);
        self.actions.push(action);
        self.rewards.push(reward);

    }

}