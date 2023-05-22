#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Strength {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl Strength {
    pub fn to_value(&self) -> char {
        match self {
            Strength::One => '1',
            Strength::Two => '2',
            Strength::Three => '3',
            Strength::Four => '4',
            Strength::Five => '5',
            Strength::Six => '6',
            Strength::Seven => '7',
            Strength::Eight => '8',
            Strength::Nine => '9',
            Strength::A => 'A',
        }
    }
}
