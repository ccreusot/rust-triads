use crate::state::State;
use crate::player::Player;
use crate::command::Command;

#[derive(Clone, Debug)]
pub struct Game {
    pub state: State,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: State::WaitingForPlayers { count: 2 },
            players: vec![],
        }
    }
}