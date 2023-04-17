
#[derive(Clone, Copy)]
enum Strength {
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  A
}

impl Strength {
  fn to_value(&self) -> char {
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
      Strength::A => 'A'    
    }
  }
}


#[derive(Clone, Copy)]
enum Owner {
  PlayerOne,
  PlayerTwo
}

impl Owner {
  fn to_sign(self) -> String {
    match self {
        Owner::PlayerOne => "o".to_string(),
        Owner::PlayerTwo => "x".to_string()
    }
  }
}

#[derive(Clone, Copy)]
struct Card {
  owner: Owner,
  top: Strength,
  left: Strength,
  bottom: Strength,
  right: Strength
}

#[derive(Clone, Copy)]
enum Cell {
  Card {
    card: Card,
  },
  Empty
}

impl Cell {
  fn to_lines(&self) -> [String; 3] {
    match self {
      Cell::Card { card } => card.to_lines(),
      Cell::Empty => [
        "       |".to_string(),
        "       |".to_string(),
        "       |".to_string(),
        ] 
    }
  }
}

impl Card {
  fn to_lines(&self) -> [String; 3] {
    [
      format!("   {}   |", self.top.to_value()),
      format!("{}  {}  {}|", self.left.to_value(), self.owner.to_sign(), self.right.to_value()),
      format!("   {}   |", self.bottom.to_value())
    ]
  }
}

#[derive(Clone, Copy)]
struct Board {
  cells: [Cell;9]
}

impl Board {
    fn new() -> Board {
        Board {
          cells: [Cell::Empty; 9]
        }
    }

    fn place_card(&self, row: usize, column: usize, card: Card) -> Board {
      let mut board_clone = self.clone(); 
      board_clone.cells[(row * 3) + column] = Cell::Card { card: card };
      board_clone
    }

    fn display(&self) -> String {
      let line_separator = "  -------------------------\n";
      let last_line = "      A       B       C\n";
      let mut board = "\n".to_string();
      board.push_str(line_separator);

      for row in 0..3 {
        let mut line: [String; 3] = ["  |".to_string(), format!("{} |", 3 - row), "  |".to_string()];
        for column in 0..3 {
          let cell = self.cells[(row * 3) + column].to_lines();
          line[0].push_str(cell[0].as_str());
          line[1].push_str(cell[1].as_str());
          line[2].push_str(cell[2].as_str());
        }
        board.push_str(line.join("\n").as_str());
        board.push_str("\n");
        board.push_str(line_separator);
      }
      board.push_str(last_line);
      println!("{}", board);
      return board;
    }
}

// Attribute used to add metadata or to apply a crates here it is the test crate
// to let us write unit test.
#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        let expected = "
  -------------------------
  |       |       |       |
3 |       |       |       |
  |       |       |       |
  -------------------------
  |       |       |       |
2 |       |       |       |
  |       |       |       |
  -------------------------
  |       |       |       |
1 |       |       |       |
  |       |       |       |
  -------------------------
      A       B       C
";
        assert_eq!(expected, Board::new().display());
    }

    #[test]
    fn test_display_board_filled_with_one_card() {
      let expected = "
  -------------------------
  |       |       |       |
3 |       |       |       |
  |       |       |       |
  -------------------------
  |       |   2   |       |
2 |       |2  o  2|       |
  |       |   2   |       |
  -------------------------
  |       |       |       |
1 |       |       |       |
  |       |       |       |
  -------------------------
      A       B       C
";
      let board = Board::new().place_card(1, 1, Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two });
      assert_eq!(expected, board.display());
    }

    #[test]
    fn test_display_board_filled_with_two_card() {
      let expected = "
  -------------------------
  |       |       |       |
3 |       |       |       |
  |       |       |       |
  -------------------------
  |       |   2   |       |
2 |       |1  o  4|       |
  |       |   3   |       |
  -------------------------
  |   5   |       |       |
1 |8  x  6|       |       |
  |   A   |       |       |
  -------------------------
      A       B       C
";

      let board = Board::new()
      .place_card(1, 1, Card {
        owner: Owner::PlayerOne, top: Strength::Two, left: Strength::One, bottom: Strength::Three, right: Strength::Four
      })
      .place_card(2, 0, Card {
        owner: Owner::PlayerTwo, top: Strength::Five, left: Strength::Eight, bottom: Strength::A, right: Strength::Six
      });
      assert_eq!(expected, board.display());
    }
}
