#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    id: String,
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
}

impl Card {
    pub fn sum(&self) -> u8 {
        (self.top + self.right + self.bottom + self.left)
    }
}
