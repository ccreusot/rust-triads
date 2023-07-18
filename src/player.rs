use crate::card::Card;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub owned_played_card: Vec<Card>,
}