#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    /// <
    Left(u8),
    /// >
    Right(u8),
    /// +
    Inc(u8),
    /// -
    Dec(u8),
    /// .
    Put,
    /// ,
    Get,
    /// [
    Open {
        // Index to set the program counter to
        jump_to: usize,
    },
    /// ]
    Close {
        // Index to set the program counter to
        jump_to: usize,
    },
}
