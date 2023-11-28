use std::fmt::{Display, Error, Formatter, Result, format};
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

    pub fn to_lines(&self, owner: String) -> [String; 3] {
        [
            format!("   {}   |", self.top),
            format!("   {}   |", self.top),
            format!("   {}   |", self.top),            
        ]
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
