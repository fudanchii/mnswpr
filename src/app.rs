use yew::prelude::*;

use crate::components::{CommandInputForm, GameBoard};

#[function_component(Mnswpr)]
pub fn mnswpr() -> Html {
    let name = use_state(|| String::new());

    let command = {
        let name = name.clone();
        Callback::from(move |input: String| name.set(input))
    };

    html! {
        <main class="container">
            <GameBoard />

            <CommandInputForm {command} />
        </main>
    }
}
