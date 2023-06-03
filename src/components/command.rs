use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CommandProps {
    pub command: Callback<String>,
}

#[function_component(CommandInputForm)]
pub fn command_input_form(props: &CommandProps) -> Html {
    let command_input_ref = use_node_ref();

    {
        let command_input_ref = command_input_ref.clone();
        use_effect(move || {
            command_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .focus()
                .unwrap();
        });
    }

    let submit_command = {
        let command_input_ref = command_input_ref.clone();
        let props = props.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let command_input = command_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap();
            props.command.emit(command_input.value().trim().to_string());
            command_input.set_value("");
        })
    };

    html! {
        <form id="cmd-form" class="row" onsubmit={submit_command}>
            <span id="cmd-container">
                <input id="cmd-input" ref={command_input_ref} class={classes!["nes-input"]} placeholder="Enter a command..." />
                <button type="submit" class={classes!["nes-btn", "is-error"]}>{"GO!"}</button>
            </span>
        </form>
    }
}
