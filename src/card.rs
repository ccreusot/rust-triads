use std::fmt::{Display, Error, Formatter, Result};
#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub id: String,
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8,
}

impl Card {
    pub fn sum(&self) -> u8 {
        (self.top + self.right + self.bottom + self.left)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "------------
|    {:02}    |
| {:02}    {:02} |
|    {:02}    |
------------",
            self.top, self.right, self.bottom, self.left
        )
    }
}

// ------------
// |    XX    |
// | XX    XX |
// |    XX    |
// ------------
