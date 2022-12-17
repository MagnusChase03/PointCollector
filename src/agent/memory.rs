#[derive(Clone)]
pub struct Memory {

    pub start_states: Vec<Vec<f64>>,
    pub actions: Vec<char>,
    pub chances: Vec<f64>,
    pub rewards: Vec<f64>

}

impl Memory {

    pub fn new() -> Memory {

        Memory {start_states: Vec::new(),
                actions: Vec::new(),
                chances: Vec::new(),
                rewards: Vec::new()}

    }

    pub fn add(&mut self, start_state: Vec<f64>, action: char, chance: f64, reward: f64) {

        self.start_states.push(start_state);
        self.actions.push(action);
        self.chances.push(chance);
        self.rewards.push(reward);

    }

}