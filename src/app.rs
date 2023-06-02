use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::CommandInputForm;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(Mnswpr)]
pub fn mnswpr() -> Html {
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
        Callback::from(move |input: String| name.set(input))
    };

    html! {
        <main class="container">
            <div id="game-board" class="board">
            </div>

            <CommandInputForm {command} />

            <p><b>{ &*command_msg }</b></p>
        </main>
    }
}
