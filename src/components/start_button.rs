use yew::prelude::*;
use yewdux::prelude::*;

use crate::exec::{GameCommandExecutor, GameState, SystemCommand, Transition};

#[function_component(GameStartResetButton)]
pub fn game_start_button() -> Html {
    let (gcx, dispatch) = use_store::<GameCommandExecutor>();

    let gamestart_callback = dispatch
        .reduce_mut_callback(move |store| store.exec(&Transition::Init(SystemCommand::Start)));

    let start_text = match gcx.current_state() {
        GameState::Init => "Start",
        _ => "Restart",
    };

    html! {
        <button type="button" class={classes!["nes-btn"]} onclick={gamestart_callback}>
            {start_text}
        </button>
    }
}
