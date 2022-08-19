#[derive(Debug, Eq, PartialEq)]
pub enum SquareContent {
    // #[default]
    Empty,
    X,
    O,
}

impl Default for SquareContent {
    fn default() -> Self {
        SquareContent::Empty
    }
}

impl From<u8> for SquareContent {
    fn from(value:u8) -> Self {
        match value {
            0 => SquareContent::Empty,
            1 => SquareContent::X,
            2 => SquareContent::O,
            v => panic!("Cannot convert {} into SquareContent", v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SquareContent;
    
    #[test]
    fn default() {
        let test_square_content: SquareContent = Default::default();
        assert_eq!(SquareContent::Empty, test_square_content);

        // Rust can infer type for Default::default() from left expression in assert_eq macro
        assert_eq!(SquareContent::Empty, Default::default());
    }

    #[test]
    fn from_into_u8() {
        // Implement T::From(U), get U.Into -> T for free.
        assert_eq!(SquareContent::X, SquareContent::from(1));
        assert_eq!(SquareContent::X, 1.into());
    }

    #[test]
    #[should_panic(expected="99")]
    fn from_fails() {
        SquareContent::from(99);
    }
}
