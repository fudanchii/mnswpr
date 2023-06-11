use serde_wasm_bindgen::to_value;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    exec::{GameCommandExecutor, TileState},
    external_binding::log,
    store::{GameState, GameStore},
};

#[function_component(GameBoard)]
pub fn game_board() -> Html {
    let (store, _) = use_store::<GameStore>();

    log(to_value(&format!(
        "[GameBoard] {:?} | {:?}",
        store.cmd(),
        store.game_state()
    ))
    .unwrap());

    match store.game_state() {
        GameState::Init => html! {
            <div class={classes!["nes-container", "is-rounded", "game-announcement"]}>
                <p>{"Let's start! You know what to do."}</p>
            </div>
        },
        GameState::DrawBoard => html! { <Board /> },
        GameState::Lose => html! { <Board /> },
        GameState::Win => html! { <Board />},
    }
}

#[function_component(Board)]
fn draw_board() -> Html {
    let (_store, dispatch) = use_store::<GameStore>();
    let (hq, _) = use_store::<GameCommandExecutor>();
    let callback = dispatch.reduce_mut_callback_with(|store, ev: MouseEvent| {
        if ev.button() == 0 {
            let button = ev.target_unchecked_into::<HtmlElement>();
            let x = button.get_attribute("data-x").unwrap();
            let y = button.get_attribute("data-y").unwrap();

            store
                .parse_command(&format!("step {}{}", x, y))
                .unwrap_or_else(|err| store.errors.push(err));
        }
    });
    let items = hq.board_map.iter().enumerate().map(|(x, row)| {
        html! {
            <tr> { for row.iter().enumerate().map(|(y, cell)| html! {
                <td> {
                    if cell.clone() == TileState::Closed {
                        html! {
                            <button class={classes!["nes-btn"]} onclick={callback.clone()} data-x={x.to_string()} data-y={y.to_string()} >
                            </button>
                        }
                    } else { html! {
                        <> {
                            if cell.clone() == TileState::Detonated {
                                "💥".to_string()
                            } else if hq.mines_map[x][y] == 99 { "💣".to_string() }
                            else { format!("{}", hq.mines_map[x][y]) }
                        } </>
                    }}
                } </td>
            }) } </tr>
        }
    });

    html! {
        <div class="nes-table-responsive">
            <table class={classes!["nes-table", "is-bordered", "is-centered"]}>
                <tbody>{ items.collect::<Html>() }</tbody>
           </table>
        </div>
    }
}
