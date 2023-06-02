use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let command_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let command_msg = use_state(|| String::new());
    {
        let command_msg = command_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("command", args).await.as_string().unwrap();
                    command_msg.set(new_msg);
                });

                || {}
            },
            name2,
        );
    }

    let command = {
        let name = name.clone();
        let command_input_ref = command_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                command_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div id="game-board" class="board">
            </div>

            <form id="cmd-form" class="row" onsubmit={command}>
                <span id="cmd-container">
                    <input id="cmd-input" ref={command_input_ref} class={classes!["nes-input"]} placeholder="Enter a command..." />
                    <button type="submit" class={classes!["nes-btn", "is-error"]}>{"GO!"}</button>
                </span>
            </form>

            <p><b>{ &*command_msg }</b></p>
        </main>
    }
}
