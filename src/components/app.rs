use yew::prelude::*;

use crate::components::{CommandInputForm, GameBoard};

#[function_component(Mnswpr)]
pub fn mnswpr() -> Html {
    html! {
        <main class="container">
            <GameBoard />
            <CommandInputForm />
        </main>
    }
}
