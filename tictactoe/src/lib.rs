#![feature(iter_intersperse)]

use std::fmt::Display;

use async_trait::async_trait;
use error::{Error, Result};
use serde::{Deserialize, Serialize};
use tabua_utils::board::grid::{Grid, GridBuilder, GridExt};

pub mod error;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Empty,
    Circle,
    Cross,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Empty => write!(f, " "),
            CellState::Circle => write!(f, "O"),
            CellState::Cross => write!(f, "X"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TicTacToeState {
    board: Grid<CellState>,
    required_sequence_length: usize,
    current_player: PlayerId,
}

impl Display for TicTacToeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.rows() {
            let row: String = row
                .iter()
                .map(ToString::to_string)
                .intersperse(" | ".to_string())
                .collect();
            writeln!(f, "{row}")?;
        }

        Ok(())
    }
}

impl TicTacToeState {
    pub fn new(board_size: usize, required_sequence_length: usize) -> Result<Self> {
        if required_sequence_length > board_size {
            return Err(Error::InvalidGameCondition);
        }

        let board = GridBuilder::new_square_grid()
            .with_rows(board_size)
            .with_columns(board_size)
            .with_initial_value(CellState::Empty)
            .build();

        Ok(Self {
            board,
            required_sequence_length,
            current_player: PlayerId::Cross,
        })
    }

    pub fn board(&self) -> &Grid<CellState> {
        &self.board
    }

    pub fn required_sequence_length(&self) -> usize {
        self.required_sequence_length
    }

    pub fn current_player(&self) -> PlayerId {
        self.current_player
    }
}

impl Default for TicTacToeState {
    fn default() -> Self {
        Self::new(3, 3).expect("Default TicTacToe game")
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerId {
    Circle,
    Cross,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Action {
    MarkBoard { player_id: PlayerId, pos: Position },
}

pub struct TicTacToeEngine {
    state: TicTacToeState,
}

impl TicTacToeEngine {
    pub fn new(state: TicTacToeState) -> Self {
        Self { state }
    }

    fn player_result(&self, player_id: PlayerId) -> EndGameState {
        let player_mark = match player_id {
            PlayerId::Circle => CellState::Circle,
            PlayerId::Cross => CellState::Cross,
        };

        let board = &self.state.board;
        let board_size = board.row_len();
        let required_sequence_length = self.state.required_sequence_length;

        // Check victory condition: row
        for rows in board.rows() {
            for window in rows.windows(required_sequence_length) {
                if window.iter().all(|x| *x == player_mark) {
                    return EndGameState::Winner(player_id);
                }
            }
        }

        // Check victory condition: column
        for col in 0..board_size {
            for start_col in 0..(board_size - required_sequence_length + 1) {
                let window: Vec<_> = board
                    .iter_column(col)
                    .skip(start_col)
                    .take(required_sequence_length)
                    .collect();
                if window.iter().all(|x| **x == player_mark)
                    && window.len() == required_sequence_length
                {
                    return EndGameState::Winner(player_id);
                }
            }
        }

        // Check victory condition: diagonals
        for r in 0..board.row_len() {
            for c in 0..board.column_len() {
                let mut count = 0;
                for (row, col) in (r..board.row_len()).zip(c..board.column_len()) {
                    if let Some(mark) = board.get(&(row, col)) {
                        if *mark == player_mark {
                            count += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if count == required_sequence_length {
                    return EndGameState::Winner(player_id);
                }

                let mut count = 0;
                for (row, col) in (0..=r).zip((0..=c).rev()) {
                    if let Some(mark) = board.get(&(row, col)) {
                        if *mark == player_mark {
                            count += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if count == required_sequence_length {
                    return EndGameState::Winner(player_id);
                }
            }
        }

        EndGameState::GameNotOver
    }

    pub fn has_empty_cell(&self) -> bool {
        for row in self.state.board.rows() {
            if row.iter().any(|x| *x == CellState::Empty) {
                return true;
            }
        }

        false
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum EndGameState {
    GameNotOver,
    Tie,
    Winner(PlayerId),
}

#[async_trait]
impl tabua_engine::Engine<'_> for TicTacToeEngine {
    type PublicState = TicTacToeState;
    type PrivateState = ();
    type PlayerId = PlayerId;
    type Action = Action;
    type EndGame = EndGameState;
    type CurrentPlayers = PlayerId;
    type Error = Error;

    async fn public_state(&self) -> Result<&Self::PublicState> {
        Ok(&self.state)
    }
    async fn private_state(&self, _user: &Self::PlayerId) -> Result<Vec<Self::PrivateState>> {
        Ok(vec![])
    }

    async fn validate_action(&self, action: &Self::Action) -> Result<()> {
        match action {
            Action::MarkBoard {
                player_id,
                pos: Position { row, column },
            } => {
                let state = &self.state;

                if *row >= state.board.row_len() || *column >= state.board.column_len() {
                    return Err(Error::CellAlreadyMarked);
                }

                if state.current_player != *player_id {
                    return Err(Error::NotPlayerTurn);
                }

                if *state.board.get(&(*row, *column)).unwrap() != CellState::Empty {
                    return Err(Error::CellAlreadyMarked);
                }
            }
        }

        if !self.has_empty_cell() {
            return Err(Error::NoMovesAvailable);
        }

        if self.results().await? != EndGameState::GameNotOver {
            return Err(Error::GameOver);
        }

        Ok(())
    }
    async fn apply_action(&mut self, action: Action) -> Result<()> {
        self.validate_action(&action).await?;

        match action {
            Action::MarkBoard {
                player_id,
                pos: Position { row, column },
            } => {
                let state = &mut self.state;

                match player_id {
                    PlayerId::Circle => {
                        *state.board.get_mut(&(row, column)).unwrap() = CellState::Circle;
                        self.state.current_player = PlayerId::Cross;
                    }
                    PlayerId::Cross => {
                        *state.board.get_mut(&(row, column)).unwrap() = CellState::Cross;
                        self.state.current_player = PlayerId::Circle;
                    }
                }

                Ok(())
            }
        }
    }

    async fn current_players(&self) -> Result<Self::CurrentPlayers> {
        Ok(self.state.current_player)
    }

    async fn results(&self) -> Result<Self::EndGame> {
        let result = match (
            self.player_result(PlayerId::Cross),
            self.player_result(PlayerId::Circle),
        ) {
            (EndGameState::GameNotOver, EndGameState::Winner(player_id))
            | (EndGameState::Winner(player_id), EndGameState::GameNotOver) => {
                EndGameState::Winner(player_id)
            }
            (EndGameState::GameNotOver, _) | (_, EndGameState::GameNotOver) => {
                EndGameState::GameNotOver
            }
            _ => unreachable!(),
        };

        if !matches!(result, EndGameState::Winner(_)) && !self.has_empty_cell() {
            return Ok(EndGameState::Tie);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use tabua_engine::Engine;

    use super::*;

    #[tokio::test]
    async fn get_initial_state() {
        let engine = TicTacToeEngine::new(TicTacToeState::default());
        let current_state = engine.public_state().await.unwrap();
        println!("{current_state}");

        assert_eq!(current_state, &TicTacToeState::default());
    }

    #[tokio::test]
    async fn one_move() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Cross,
                pos: Position::new(0, 0),
            })
            .await
            .unwrap();

        let new_state = engine.public_state().await.unwrap();

        let expected = {
            let mut state = TicTacToeState::default();
            *state.board.get_mut(&(0, 0)).unwrap() = CellState::Cross;
            state.current_player = PlayerId::Circle;
            state
        };

        println!("{new_state}");
        assert_eq!(new_state, &expected);
    }

    #[tokio::test]
    async fn two_moves() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Cross,
                pos: Position::new(0, 0),
            })
            .await
            .unwrap();
        engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Circle,
                pos: Position::new(1, 0),
            })
            .await
            .unwrap();

        let new_state = engine.public_state().await.unwrap();

        let expected = {
            let mut state = TicTacToeState::default();
            *state.board.get_mut(&(0, 0)).unwrap() = CellState::Cross;
            *state.board.get_mut(&(1, 0)).unwrap() = CellState::Circle;
            state.current_player = PlayerId::Cross;
            state
        };

        println!("{new_state}");
        assert_eq!(new_state, &expected);
    }

    #[tokio::test]
    async fn invalid_move() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Cross,
                pos: Position::new(0, 0),
            })
            .await
            .unwrap();

        assert!(engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Circle,
                pos: Position::new(0, 0),
            })
            .await
            .is_err())
    }

    #[tokio::test]
    async fn wrong_player() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        assert!(engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Circle,
                pos: Position::new(0, 0),
            })
            .await
            .is_err());
    }

    #[tokio::test]
    async fn game_not_over() {
        let engine = TicTacToeEngine::new(TicTacToeState::default());
        assert_eq!(engine.results().await.unwrap(), EndGameState::GameNotOver);
    }

    #[tokio::test]
    async fn player_cross_victory_row() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        assert_eq!(engine.results().await.unwrap(), EndGameState::GameNotOver);
        engine.state.board = Grid::new_square_grid(vec![
            vec![CellState::Cross, CellState::Cross, CellState::Cross],
            vec![CellState::Empty, CellState::Empty, CellState::Empty],
            vec![CellState::Empty, CellState::Empty, CellState::Empty],
        ]);
        assert_eq!(
            engine.results().await.unwrap(),
            EndGameState::Winner(PlayerId::Cross)
        );
    }

    #[tokio::test]
    async fn player_cross_victory_column() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        assert_eq!(engine.results().await.unwrap(), EndGameState::GameNotOver);
        engine.state.board = Grid::new_square_grid(vec![
            vec![CellState::Cross, CellState::Empty, CellState::Empty],
            vec![CellState::Cross, CellState::Empty, CellState::Empty],
            vec![CellState::Cross, CellState::Empty, CellState::Empty],
        ]);
        assert_eq!(
            engine.results().await.unwrap(),
            EndGameState::Winner(PlayerId::Cross)
        );
    }

    #[tokio::test]
    async fn player_cross_victory_diagonal() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        assert_eq!(engine.results().await.unwrap(), EndGameState::GameNotOver);
        engine.state.board = Grid::new_square_grid(vec![
            vec![CellState::Cross, CellState::Empty, CellState::Empty],
            vec![CellState::Empty, CellState::Cross, CellState::Empty],
            vec![CellState::Empty, CellState::Empty, CellState::Cross],
        ]);
        assert_eq!(
            engine.results().await.unwrap(),
            EndGameState::Winner(PlayerId::Cross)
        );
    }

    #[tokio::test]
    async fn player_circle_victory_row() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        assert_eq!(engine.results().await.unwrap(), EndGameState::GameNotOver);
        engine.state.board = Grid::new_square_grid(vec![
            vec![CellState::Empty, CellState::Empty, CellState::Empty],
            vec![CellState::Circle, CellState::Circle, CellState::Circle],
            vec![CellState::Empty, CellState::Empty, CellState::Empty],
        ]);
        assert_eq!(
            engine.results().await.unwrap(),
            EndGameState::Winner(PlayerId::Circle)
        );
    }

    #[tokio::test]
    async fn player_circle_victory_column() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        assert_eq!(engine.results().await.unwrap(), EndGameState::GameNotOver);
        engine.state.board = Grid::new_square_grid(vec![
            vec![CellState::Empty, CellState::Circle, CellState::Empty],
            vec![CellState::Empty, CellState::Circle, CellState::Empty],
            vec![CellState::Empty, CellState::Circle, CellState::Empty],
        ]);
        assert_eq!(
            engine.results().await.unwrap(),
            EndGameState::Winner(PlayerId::Circle)
        );
    }

    #[tokio::test]
    async fn player_circle_victory_diagonal() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        assert_eq!(engine.results().await.unwrap(), EndGameState::GameNotOver);
        engine.state.board = Grid::new_square_grid(vec![
            vec![CellState::Empty, CellState::Empty, CellState::Circle],
            vec![CellState::Empty, CellState::Circle, CellState::Empty],
            vec![CellState::Circle, CellState::Empty, CellState::Empty],
        ]);
        assert_eq!(
            engine.results().await.unwrap(),
            EndGameState::Winner(PlayerId::Circle)
        );
    }

    #[tokio::test]
    async fn no_moves_available() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::default());
        assert_eq!(engine.results().await.unwrap(), EndGameState::GameNotOver);
        engine.state.board = Grid::new_square_grid(vec![
            vec![CellState::Circle, CellState::Cross, CellState::Circle],
            vec![CellState::Cross, CellState::Cross, CellState::Circle],
            vec![CellState::Circle, CellState::Circle, CellState::Cross],
        ]);
        let action = Action::MarkBoard {
            player_id: PlayerId::Circle,
            pos: Position::new(0, 0),
        };
        assert!(engine.validate_action(&action).await.is_err());
        assert!(engine.apply_action(action).await.is_err());
        assert_eq!(engine.results().await.unwrap(), EndGameState::Tie);
    }
}
