use crate::board::Board;
use crate::card::Card;
use crate::owner::Owner;
use crate::strength::Strength;

trait HandGenerator {
    fn generate_hand(&self, owner: Owner) -> Vec<Card>;
}

struct RandomHandGenerator;

impl HandGenerator for RandomHandGenerator {
    fn generate_hand(&self, _: Owner) -> Vec<Card> {
        vec![]
    }
}

struct NotRandomHandGenerator {
    hand: Vec<Card>,
}

impl NotRandomHandGenerator {
    fn new(hand: Vec<Card>) -> NotRandomHandGenerator {
        NotRandomHandGenerator {
            hand,
        }
    }
}

impl HandGenerator for NotRandomHandGenerator {
    fn generate_hand(&self, owner: Owner) -> Vec<Card> {
        self.hand.clone().into_iter().map(|card| Card { owner, ..card }).collect()
    }
}

#[derive(Clone)]
struct Player {
    name: String,
    score: u32,
    hand: Vec<Card>,
}

impl Player {
    fn remove_card(&self, card: Card) -> Player {
        let mut player = self.clone();
        player.hand = player.hand.into_iter().filter(|c| *c != card).collect();
        player
    }
}

#[derive(Clone)]
struct Game {
    player_a: Player,
    player_b: Player,
    board: Board,
}

impl Game{
    fn new(hand_generator: impl HandGenerator) -> Game {
        Game {
            player_a: Player {
                name: "Player A".to_string(),
                score: 0,
                hand: hand_generator.generate_hand(Owner::PlayerOne),
            },
            player_b: Player {
                name: "Player B".to_string(),
                score: 0,
                hand: hand_generator.generate_hand(Owner::PlayerTwo),
            },
            board: Board::new(),
        }
    }

    fn player_input(&self, row: usize, column: usize, card: Card) -> Game {
        // Card we have the current owner and player that is playing
        let board = self.board.place_card(row, column, card);
        // We need to get the cells that are neighbors of the cell we are placing the card
        // TODO 

        if card.owner == Owner::PlayerOne {
            let mut game = self.clone();
            game.player_a =  self.player_a.remove_card(card);
            game.board = board;
            game
        } else {
            let mut game = self.clone();
            game.player_b =  self.player_b.remove_card(card);
            game.board = board;
            game
        }
    }

    fn get_winner(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod game_tests {
    use crate::card::Card;
    use crate::game::{Game, NotRandomHandGenerator};
    use crate::owner::Owner;
    use crate::strength::Strength;
    use crate::cell::Cell;

    #[test]
    fn test_has_2_players() {
        let hand_generator = NotRandomHandGenerator::new(vec![
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
        let game = Game::new(hand_generator);
        assert_eq!(game.player_a.name, "Player A");
        assert_eq!(game.player_a.hand, vec![
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
        assert_eq!(game.player_b.name, "Player B");
        assert_eq!(game.player_b.hand, vec![
            Card { owner: Owner::PlayerTwo, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
    }

    #[test]
    fn test_player_one_is_the_current_player_at_start() {
        let hand_generator = NotRandomHandGenerator::new(vec![
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
        let mut game = Game::new(hand_generator);

        game = game.player_input(1, 1, game.player_a.hand[0]);

        if let Cell::Card { card } = game.board.get_cell(1, 1) {
            assert_eq!(card.owner, Owner::PlayerOne);
        }

        assert_eq!(game.player_a.hand, vec![]);
    }

    #[test]
    fn test_player_two_is_the_current_player_after_player_one() {
        let hand_generator = NotRandomHandGenerator::new(vec![
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
        let mut game = Game::new(hand_generator);

        game = game.player_input(1, 1, game.player_a.hand[0]);
        game = game.player_input(1, 2, game.player_b.hand[0]);

        if let Cell::Card { card } = game.board.get_cell(1, 2) {
            assert_eq!(card.owner, Owner::PlayerTwo);
        }

        assert_eq!(game.player_b.hand, vec![]);
    }

    #[test]
    fn test_when_all_card_have_played_end_of_the_game_if_both_player_have_the_same_number_of_cards_on_the_board_it_should_be_a_tie() {
        let hand_generator = NotRandomHandGenerator::new(vec![
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
        let mut game = Game::new(hand_generator);

        game = game.player_input(1, 1, game.player_a.hand[0]);
        game = game.player_input(1, 2, game.player_b.hand[0]);

        assert_eq!(game.player_a.hand, vec![]);
        assert_eq!(game.player_b.hand, vec![]);
        assert_eq!(game.get_winner(), None);
    }

    #[test]
    fn test_when_all_card_have_played_end_of_the_game_if_player_one_has_more_cards_on_the_board_than_player_two_player_one_win() {
        let hand_generator = NotRandomHandGenerator::new(vec![
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::One },
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
        let mut game = Game::new(hand_generator);

        game = game.player_input(0, 0, game.player_a.hand[0]);
        game = game.player_input(0, 1, game.player_b.hand[1]);
        game = game.player_input(0, 2, game.player_a.hand[0]);
        game = game.player_input(1, 0, game.player_b.hand[0]);
        game = game.player_input(1, 1, game.player_a.hand[0]);
        game = game.player_input(1, 2, game.player_b.hand[0]);

        assert_eq!(game.player_a.hand, vec![]);
        assert_eq!(game.player_b.hand, vec![]);
        assert_eq!(game.get_winner(), Some("Player A".to_string()));
    }
}