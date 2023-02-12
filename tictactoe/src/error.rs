use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Impossible endgame condition: required sequence length must be less or equal the board length")]
    InvalidGameCondition,
    #[error("cell already marked")]
    CellAlreadyMarked,
    #[error("Playing out of turn")]
    NotPlayerTurn,
    #[error("No moves available")]
    NoMovesAvailable,
    #[error("Game over")]
    GameOver,
}

pub type Result<T> = std::result::Result<T, Error>;
