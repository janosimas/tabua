use components::Grid;
use dioxus::prelude::*;
use futures_util::StreamExt;
use log::debug;
#[allow(unused)]
use log::{error, info, warn};
use tabua_engine::Engine;
use tictactoe::{Action, PlayerId, TicTacToeEngine, TicTacToeState};

mod components;

#[derive(Debug, Clone, PartialEq)]
enum Signal {
    Action(Action),
    Refresh,
}

#[derive(Debug, Default)]
struct UiState {
    game_state: Option<TicTacToeState>,
    current_player: Option<PlayerId>,
}

fn main() {
    setup_logger().unwrap();
    dioxus_desktop::launch(app);
}

async fn engine_service(mut rx: UnboundedReceiver<Signal>, ui_state: UseState<UiState>) {
    let mut engine = TicTacToeEngine::new(TicTacToeState::default());

    while let Some(msg) = rx.next().await {
        info!("signal received: {:?}", msg);
        let result = match msg {
            Signal::Action(action) => engine.apply_action(action).await,
            Signal::Refresh => Ok(()),
        };

        if let Err(err) = result {
            error!("{}", err);
            continue;
        }

        let current_state = engine.public_state().await;
        let current_player = engine.current_players().await;

        let mut new_ui_state = UiState::default();

        if let Ok(current_state) = current_state {
            info!("New state set");
            new_ui_state.game_state = Some(current_state.clone());
        } else {
            warn!("Fail to get new state");
        }

        if let Ok(current_players) = current_player {
            info!("Current players set");
            new_ui_state.current_player = Some(current_players);
        } else {
            warn!("Fail to get current players");
        }

        ui_state.set(new_ui_state);
    }
}

fn app(cx: Scope) -> Element {
    let ui_state = use_state(cx, UiState::default);
    let sender = use_coroutine(cx, |rx| {
        to_owned![ui_state];
        engine_service(rx, ui_state)
    });

    if let Some(board) = ui_state
        .game_state
        .as_ref()
        .map(|state| state.board().clone())
    {
        cx.render(rsx!(
            Grid {
                state: board,
                player: if let Some(players) = &ui_state.current_player {
                    *players
                } else {
                    PlayerId::Cross
                }
            }
            p { format!("{:?}", ui_state.current_player) }
        ))
    } else {
        cx.render(rsx!(
            h2 { "error getting game board" }
            button {
                onclick: |_| {
                    debug!("refresh!!!");
                    sender.send(Signal::Refresh);
                    cx.needs_update();
                },
                "refresh"
            }
        ))
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
