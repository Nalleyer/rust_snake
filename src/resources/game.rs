#[derive(Debug, PartialEq)]
pub enum State {
    Loading,
    Main,
}

#[derive(Debug)]
pub struct Game {
    current_state: State,
}

impl Game {
    pub fn new(current_state: State) -> Self {
        Game { current_state }
    }

    pub fn get_state(&self) -> &State {
        &self.current_state
    }

    pub fn set_state(&mut self, state: State) {
        self.current_state = state;
    }
}
