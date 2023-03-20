
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

#[derive(Clone, Copy)]
enum Owner {
  PlayerOne,
  PlayerTwo
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
struct Board {
  cells: [Option<Card>;9]
}

impl Board {
    fn new() -> Board {
        Board {
          cells: [Option::None; 9]
        }
    }

    fn place_card(&self, row: usize, column: usize, card: Card) -> Board {
      let mut board_clone = self.clone(); 
      board_clone.cells[(row * column) % 3] = Some(card);
      board_clone
    }

    // pub(self) fn display_cells(card: Option<Card>) -> String {
    //   if let Some(card) = card {
    //     let top = format!("   {}   ", 2);
    //     let center = format!("{}  {}  {}", 2, "o", 2);
    //     let bottom = format!("   {}   ", 2);
    //     return "
    //     {top}
    //     {center}
    //     {bottom}
    //     ".to_string();
    //   } else {
    //     return "".to_string();
    //   }
    // }

    // pub(self) fn display_row() -> String {
    //   return "".to_string();      
    // }

    fn display(&self) -> String {
      for (index, cell) in self.cells.iter().enumerate() {
        // TODO : Fill this with the algorithm to pretty print the beautifull board bellow
      }
        return "
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
".to_string();
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
}
