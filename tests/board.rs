struct Board {
}

impl Board {
    fn new() -> Board {
        Board {}
    }

    fn display(&self) -> String {
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
}
