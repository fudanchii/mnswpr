use yew::prelude::*;

use crate::components::GameBoard;

#[function_component(Mnswpr)]
pub fn mnswpr() -> Html {
    html! {
        <main class="container">
            <GameBoard />
        </main>
    }
}
