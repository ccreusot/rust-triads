enum Command {
    Register { name: String },
    Select { card: Card },
    ChoosePlayer,
    Play { card: Card, x: u8, y: u8 },
    Check { x: u8, y: u8 },
}

#[derive(Clone, PartialEq, Debug)]
enum State {
    WaitingForPlayers { count: u8 },
    WaitingForCards { player: u8 },
}

#[derive(Clone, Debug)]
struct Game {
    state: State,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: State::WaitingForPlayers { count: 2 },
        }
    }

    pub fn push(&self, command: Command) -> Game {
        match command {
            Command::Register { name } => self.register_player(name), 
            _ => self.clone()
        }
    }

    fn register_player(&self, name: String) -> Game {
        let State::WaitingForPlayers { count } = self.state;
        return Game {
            state: State::WaitingForPlayers { count: count - 1 },
        }
    }
}

struct Card {
    id: String,
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
}

struct Player {
    id: u8,
    score: u32,
    name: String,
    hand: Vec<Card>,
    owned_played_card: Vec<Card>,
}

struct Board {
    cards: Vec<Vec<Option<Card>>>,
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_creation_game_should_be_in_waiting_for_players_state() {
        let game = Game::new();

        assert_eq!(game.state, State::WaitingForPlayers { count: 2 });
    }

    #[test]
    fn when_we_push_one_registering_command_to_the_game_we_should_get_a_state_waiting_for_player_one() {
        let mut game = Game::new();

        game = game.push(Command::Register { name: "Player 1".to_string() });

        assert_eq!(game.state, State::WaitingForPlayers { count: 1 });
    }

    #[test]
    fn when_we_push_two_registering_commands_to_the_game_we_should_get_a_state_waiting_for_card_for_player_one() {
        let mut game = Game::new();

        game = game.push(Command::Register { name: "Player 1".to_string() });
        game = game.push(Command::Register { name: "Player 2".to_string() });

        //assert_eq!(game.state, State::WaitingForPlayers { count:  });
    }

    // TODO : Test with WaitingForPlayer with any other command what should happen ?
}