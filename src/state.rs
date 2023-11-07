use crate::card::Card;

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    WaitingForPlayers { count: u8 },
    // Todo: When Player count decrement we generate a new list of card for the next player
    WaitingForCards { player_count: u8, deck: Vec<Card> },
    WaitingForPlayerToPlay { player_name: String },
    EndOfGame { scores: (u8, u8), winner: String },
}
