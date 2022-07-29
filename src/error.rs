#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("There is an unmatched bracket at instruction {0}")]
    UnmatchedBracket(usize),
    #[error("The program went too far to the left.")]
    OutOfBounds,
}
