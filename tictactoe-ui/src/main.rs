use color_eyre::Result;
use tabua_engine::*;
use tabua_utils::board::grid::GridExt;
use tictactoe::{Action, CellState, PlayerId, Position, TicTacToeEngine, TicTacToeState};

use std::rc::Rc;

slint::slint!(import { Cell } from "ui/main.slint";);

#[tokio::main]
async fn main() -> Result<()> {
    let mut engine = TicTacToeEngine::new(TicTacToeState::default());
    engine
        .apply_action(Action::MarkBoard {
            player_id: PlayerId::Cross,
            pos: Position::new(0, 0),
        })
        .await?;

    let state = engine.public_state().await.unwrap();
    for row in state.board().rows() {
        // row.
    }

    // let model =  Rc::new(slint::VecModel::<Cell>::from(vec![Cell::default(); 15]));
    // MainWindow::new().run();

    let state = CellState::Circle;
    let cell = Cell::new();
    cell.set_state(
        match state {
            CellState::Empty => "X",
            CellState::Cross => "0",
            CellState::Circle => "",
        }
        .into(),
    );
    cell.on_clicked(cell_clicked);

    Ok(())
}

fn cell_clicked(row: i32, col: i32) {
    println!("clicked")
}
