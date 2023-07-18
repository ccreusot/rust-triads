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

    pub fn push(&self, command: Command) -> Game {
        match self.state {
            State::WaitingForPlayers { .. } => {
                self.handle_command_for_waiting_for_player_state(command)
            }
            _ => panic!("State {:?}: Not implemented yet", self.state),
        }
    }

    fn handle_command_for_waiting_for_player_state(&self, command: Command) -> Game {
        match command {
            Command::Register { name } => self.register_player(name),
            _ => self.clone(),
        }
    }
}