use crate::Signal;

use dioxus::prelude::*;
use log::debug;
use tabua_utils::board;
use tabua_utils::board::grid::GridExt;
use tictactoe::{Action, CellState, PlayerId, Position};

#[derive(PartialEq, Props)]
pub(crate) struct GridProps {
    state: board::grid::Grid<CellState>,
    player: PlayerId,
}

#[allow(non_snake_case)]
pub(crate) fn Grid(cx: Scope<GridProps>) -> Element {
    let GridProps { state, player } = cx.props;
    let engine = use_coroutine_handle::<Signal>(cx).unwrap();

    let count = state.row_len().min(state.column_len());
    let size = 60 / count;

    let rows = state.rows().enumerate().map(|(i, row)| {
        let cells = row.iter().enumerate().map({
            to_owned![size];
            move |(j, cell)| {
                to_owned![size];
                rsx! {
                    td {
                        width: "{size}vw",
                        height: "{size}vw",

                        border_width: "1px",
                        border_style: "solid",
                        border_color: "black",

                        text_align: "center",

                        font_size: "{size as f64 * 0.5}vw",

                        onclick: move |evt| {
                            evt.stop_propagation();
                            debug!("send move");
                            engine.send(Signal::Action(Action::MarkBoard {
                                player_id: *player,
                                pos: Position::new(i, j),
                            }));
                        },
                        cell.to_string()
                    }
                }
            }
        });

        rsx! { tr { cells } }
    });

    cx.render(rsx! {
        table {
            border:"1",
            tbody { rows }
        }
    })
}
