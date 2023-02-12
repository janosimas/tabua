use dioxus::prelude::*;
use log::debug;
use tictactoe::{Action, CellState, PlayerId, Position};

use crate::Signal;

#[derive(PartialEq, Props)]
pub(crate) struct CellProps<'a> {
    state: &'a CellState,
    current_player: PlayerId,
    column: usize,
    row: usize,
}

#[allow(non_snake_case)]
pub(crate) fn Cell<'a>(cx: Scope<'a, CellProps<'a>>) -> Element<'a> {
    let CellProps {
        state,
        current_player,
        column,
        row,
    } = *cx.props;

    let engine = use_coroutine_handle::<Signal>(cx).unwrap();

    cx.render(rsx! {
        div {
            width: "15px",
            height: "15px",

            border_width: "1px",
            border_style: "solid",
            border_color: "black",

            text_align: "center",
            onclick: move |_| {
                debug!("send move");
                engine.send(Signal::Action(Action::MarkBoard {
                    player_id: current_player,
                    pos: Position::new(column, row),
                }))
            },
            "{state}"
        }
    })
}
