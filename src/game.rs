use crate::board::Board;
use crate::player::Player;
use crate::state::State;

#[derive(Clone, Debug)]
pub struct Game {
    pub state: State,
    pub players: Vec<Player>,
    pub board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: State::WaitingForPlayers { count: 2 },
            players: vec![],
            board: Board::new(),
        }
    }
}
