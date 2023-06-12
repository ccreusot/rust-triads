use crate::card::Card;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Card { card: Card },
    Empty,
}

impl Cell {
    pub fn to_lines(&self) -> [String; 3] {
        match self {
            Cell::Card { card } => card.to_lines(),
            Cell::Empty => [
                "       |".to_string(),
                "       |".to_string(),
                "       |".to_string(),
            ],
        }
    }
}
