#![feature(iter_intersperse)]
#![feature(generic_associated_types)]

use std::fmt::Display;

use color_eyre::eyre::bail;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TicTacToeState {
    board: Vec<Vec<CellState>>,
    current_player: PlayerId,
}

impl Display for TicTacToeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.board {
            let line: String = line
                .iter()
                .map(ToString::to_string)
                .intersperse(" | ".to_string())
                .collect();
            writeln!(f, "{line}")?;
        }

        Ok(())
    }
}

impl TicTacToeState {
    pub fn new(board_size: usize) -> Self {
        let board = std::iter::repeat_with(|| {
            std::iter::repeat(CellState::Empty)
                .take(board_size)
                .collect()
        })
        .take(board_size)
        .collect();

        Self {
            board,
            current_player: PlayerId::Cross,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum PlayerId {
    Circle,
    Cross,
}

pub enum Action {
    MarkBoard {
        player_id: PlayerId,
        line: usize,
        column: usize,
    },
}

pub struct TicTacToeEngine {
    state: TicTacToeState,
}

impl TicTacToeEngine {
    pub fn new(state: TicTacToeState) -> Self {
        Self { state }
    }
}

impl tabua_engine::Engine for TicTacToeEngine {
    type PublicState<'a> = TicTacToeState where Self: 'a;
    type PrivateState<'a> = ();
    type PlayerId = PlayerId;
    type Action = Action;
    type EndGame = ();

    fn public_state(&self) -> &Self::PublicState<'_> {
        &self.state
    }
    fn private_state(&self, _user: &Self::PlayerId) -> &'_ [&Self::PublicState<'_>] {
        &[]
    }

    fn is_action_valid(&self, action: &Self::Action) -> Result<()> {
        match action {
            Action::MarkBoard {
                player_id,
                line,
                column,
            } => {
                let state = &self.state;

                if *line >= state.board.len() || *column >= state.board[0].len() {
                    bail!("Invalid move, cell already marked");
                }

                if state.current_player != *player_id {
                    bail!("Playing out of turn");
                }

                if state.board[*line][*column] != CellState::Empty {
                    bail!("Invalid move, cell already marked");
                }
            }
        }

        Ok(())
    }
    fn apply_action(&mut self, action: Action) -> Result<&Self::PublicState<'_>> {
        self.is_action_valid(&action)?;

        match action {
            Action::MarkBoard {
                player_id,
                line,
                column,
            } => {
                let state = &mut self.state;

                match player_id {
                    PlayerId::Circle => {
                        state.board[line][column] = CellState::Circle;
                        self.state.current_player = PlayerId::Cross;
                    }
                    PlayerId::Cross => {
                        state.board[line][column] = CellState::Cross;
                        self.state.current_player = PlayerId::Circle;
                    }
                }

                Ok(&self.state)
            }
        }
    }

    fn current_players(&self) -> Vec<Self::PlayerId> {
        vec![self.state.current_player]
    }

    fn is_over(&self) -> bool {
        todo!()
    }

    fn results(&self) -> Result<Self::EndGame> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use tabua_engine::Engine;

    use super::*;

    #[test]
    fn get_initial_state() {
        let engine = TicTacToeEngine::new(TicTacToeState::new(3));
        let current_state = engine.public_state();
        println!("{current_state}");

        assert_eq!(*current_state, TicTacToeState::new(3));
    }

    #[test]
    fn one_move() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::new(3));
        let new_state = engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Cross,
                line: 0,
                column: 0,
            })
            .unwrap();

        let expected = {
            let mut state = TicTacToeState::new(3);
            state.board[0][0] = CellState::Cross;
            state.current_player = PlayerId::Circle;
            state
        };

        println!("{new_state}");
        assert_eq!(*new_state, expected);
    }

    #[test]
    fn two_moves() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::new(3));
        engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Cross,
                line: 0,
                column: 0,
            })
            .unwrap();
        let new_state = engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Circle,
                line: 1,
                column: 0,
            })
            .unwrap();

        let expected = {
            let mut state = TicTacToeState::new(3);
            state.board[0][0] = CellState::Cross;
            state.board[1][0] = CellState::Circle;
            state.current_player = PlayerId::Cross;
            state
        };

        println!("{new_state}");
        assert_eq!(*new_state, expected);
    }

    #[test]
    fn wrong_player() {
        let mut engine = TicTacToeEngine::new(TicTacToeState::new(3));
        assert!(engine
            .apply_action(Action::MarkBoard {
                player_id: PlayerId::Circle,
                line: 0,
                column: 0,
            })
            .is_err());
    }
}
