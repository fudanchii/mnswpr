use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::TimerDisplay,
    exec::{GameCommandExecutor, GameState},
};

#[function_component(CommandInputForm)]
pub fn command_input_form() -> Html {
    let command_input_ref = use_node_ref();
    let (hq, dispatch) = use_store::<GameCommandExecutor>();

    {
        let command_input_ref = command_input_ref.clone();
        use_effect(move || {
            command_input_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .focus()
                .unwrap();
        });
    }

    let input_command = dispatch.reduce_callback_with(|store, e: KeyboardEvent| {
        if e.key() == "Enter" {
            let mut gcx = (*store).clone();
            let command_input: HtmlInputElement = e.target_unchecked_into();
            gcx.parse_command(command_input.value().trim())
                .map(|cmd| gcx.exec(&cmd))
                .unwrap();
            command_input.set_value("");
            return gcx.into();
        }
        store
    });

    let placeholder = match *hq.current_state() {
        GameState::Win => "YOU WIN!",
        GameState::Lose => "GAME OVER",
        GameState::Paused => "zzZ...",
        _ => "type command...",
    };

    html! {
        <div id="cmd-form">
            <span id="cmd-container">
                <input id="cmd-input"
                    ref={command_input_ref}
                    class={classes!["nes-input"]}
                    {placeholder}
                    onkeypress={input_command} />
                <TimerDisplay />
            </span>
        </div>
    }
}
