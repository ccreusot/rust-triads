
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// Attribute used to add metadata or to apply a crates here it is the test crate
// to let us write unit test.
#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_fail() {
        assert_eq!(add(1, 2), 4);
    }
}