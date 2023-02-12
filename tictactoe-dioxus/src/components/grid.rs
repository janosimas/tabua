use super::Cell;
use dioxus::prelude::*;
use tabua_utils::board;
use tabua_utils::board::grid::GridExt;
use tictactoe::{CellState, PlayerId};

#[derive(PartialEq, Props)]
pub(crate) struct GridProps {
    state: board::grid::Grid<CellState>,
    player: PlayerId,
}

#[allow(non_snake_case)]
pub(crate) fn Grid(cx: Scope<GridProps>) -> Element {
    let GridProps { state, player } = cx.props;
    cx.render(rsx! (
        div {
            display: "flex",
            flex_wrap: "nowrap",
            flex_direction: "column",
            width: "100%",
            height: "100%",
            div{
                display: "flex",
                flex_direction: "row",
                state
                    .iter_row(0)
                    .enumerate()
                    .map(|(row, cell_state)| rsx!(Cell { state: cell_state, column: 0, row: row, current_player: *player }))
            }
            div{
                display: "flex",
                flex_direction: "row",
                state
                    .iter_row(1)
                    .enumerate()
                    .map(|(row, cell_state)| rsx!(Cell { state: cell_state, column: 1, row: row, current_player: *player }))
            }
            div{
                display: "flex",
                flex_direction: "row",
                state
                    .iter_row(2)
                    .enumerate()
                    .map(|(row, cell_state)| rsx!(Cell { state: cell_state, column: 2, row: row, current_player: *player }))
            }
        }
    ))
}
