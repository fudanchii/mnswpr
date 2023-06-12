use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    exec::{GameCommandExecutor, TileState},
    store::{GameState, GameStore}, external_binding::log,
};

#[function_component(GameBoard)]
pub fn game_board() -> Html {
    let (gcx, _) = use_store::<GameCommandExecutor>();

    log(format!("state painting {:?}", gcx.state).into());

    match gcx.state {
        GameState::Init => html! {
            <div class={classes!["nes-container", "is-rounded", "game-announcement"]}>
                <p>{"Let's start! You know what to do."}</p>
            </div>
        },
        _ => html !{ <Board /> },
    }
}

#[function_component(Board)]
fn draw_board() -> Html {
    let (_, dispatch) = use_store::<GameStore>();
    let (hq, _) = use_store::<GameCommandExecutor>();
    let mut btn_classes = vec!["nes-btn"];
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

    log(format!("board drawing {:?}", hq.state).into());

    if hq.state == GameState::Lose || hq.state == GameState::Win {
        btn_classes.push("is-disabled");
    }

    let items = hq.board_map.iter().enumerate().map(|(y, row)| {
        html! {
            <tr> { for row.iter().enumerate().map(|(x, cell)| html! {
                <td class={classes!["mine-cell"]}> {
                    if cell.clone() == TileState::Closed {
                        html! {
                            <button class={btn_classes.clone()} onclick={callback.clone()} data-x={(x+1).to_string()} data-y={(y+1).to_string()} >
                            </button>
                        }
                    } else { html! {
                        <> {
                            if cell.clone() == TileState::Flagged { "🚩".to_string() }
                            else if hq.mines_map[y][x] == 0 { "".to_string() }
                            else if cell.clone() == TileState::Detonated {
                                "💣".to_string()
                            } else if hq.mines_map[y][x] == 99 { "💥".to_string() }
                            else { format!("{}", hq.mines_map[y][x]) }
                        } </>
                    }}
                } </td>
            }) } </tr>
        }
    });

    html! {
        <div class="nes-table-responsive">
            <table class={classes!["mines-field", "nes-table", "is-bordered", "is-centered"]}>
                <tbody>{ items.collect::<Html>() }</tbody>
           </table>
        </div>
    }
}
