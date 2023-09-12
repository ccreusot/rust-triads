use crate::card::Card;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}