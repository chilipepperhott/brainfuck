use crate::op::Op;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    /// <
    Left,
    /// >
    Right,
    /// +
    Inc,
    /// -
    Dec,
    /// .
    Put,
    /// ,
    Get,
    /// [
    Open,
    /// ]
    Close,
}

impl Token {
    pub fn new_from_char(c: char) -> Option<Self> {
        let f = match c {
            '<' => Self::Left,
            '>' => Self::Right,
            '+' => Self::Inc,
            '-' => Self::Dec,
            '.' => Self::Put,
            ',' => Self::Get,
            '[' => Self::Open,
            ']' => Self::Close,
            _ => return None,
        };

        Some(f)
    }

    pub fn lex_from_iter<'a>(
        iter: impl Iterator<Item = char> + 'a,
    ) -> impl Iterator<Item = Self> + 'a {
        iter.filter_map(Self::new_from_char)
    }
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn lexes_basic() {
        assert_eq!(
            Token::lex_from_iter("<>+-.,[]".chars()).collect::<Vec<_>>(),
            vec![
                Token::Left,
                Token::Right,
                Token::Inc,
                Token::Dec,
                Token::Put,
                Token::Get,
                Token::Open,
                Token::Close,
            ]
        )
    }

    #[test]
    fn skips_other_chars() {
        assert_eq!(
            Token::lex_from_iter("<>+-gelloasjdhhg.,[]".chars()).collect::<Vec<_>>(),
            vec![
                Token::Left,
                Token::Right,
                Token::Inc,
                Token::Dec,
                Token::Put,
                Token::Get,
                Token::Open,
                Token::Close,
            ]
        )
    }
}

impl From<Op> for Token {
    fn from(op: Op) -> Self {
        match op {
            Op::Left(_) => Token::Left,
            Op::Right(_) => Token::Right,
            Op::Inc(_) => Token::Inc,
            Op::Dec(_) => Token::Dec,
            Op::Put => Token::Put,
            Op::Get => Token::Get,
            Op::Open { .. } => Token::Close,
            Op::Close { .. } => Token::Close,
        }
    }
}
