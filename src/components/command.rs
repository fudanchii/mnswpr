use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    exec::GameCommandExecutor,
    external_binding::log,
    store::{GameState, GameStore},
};

#[function_component(CommandInputForm)]
pub fn command_input_form() -> Html {
    let command_input_ref = use_node_ref();
    let (hq, _) = use_store::<GameCommandExecutor>();
    let (_, dispatch) = use_store::<GameStore>();

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

    let input_command = dispatch.reduce_mut_callback_with(|store, e: KeyboardEvent| {
        if e.key() == "Enter" {
            let command_input: HtmlInputElement = e.target_unchecked_into();
            log(format!("entered {}", command_input.value().trim()).into());
            store
                .parse_command(command_input.value().trim())
                .unwrap_or_else(|err| store.errors.push(err));
            command_input.set_value("");
        }
    });

    let placeholder = if *hq.current_state() == GameState::Win {
        "YOU WIN!"
    } else if *hq.current_state() == GameState::Lose {
        "GAME OVER"
    } else {
        "insert command..."
    };

    html! {
        <div id="cmd-form">
            <span id="cmd-container">
                <input id="cmd-input"
                    ref={command_input_ref}
                    class={classes!["nes-input"]}
                    {placeholder}
                    onkeypress={input_command} />
            </span>
        </div>
    }
}
