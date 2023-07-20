#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
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
