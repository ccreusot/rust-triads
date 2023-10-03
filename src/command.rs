use crate::card::Card;

#[derive(Clone, Debug)]
pub enum Command {
    Register { name: String },
    SelectCard { card_id: String },
    ChoosePlayer,
    Play { card_id: String, x: u8, y: u8 },
    Check { x: u8, y: u8 },
}
