use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    exec::{GameCommandExecutor, TileState},
    store::{GameState, GameStore}, components::CommandInputForm,
};

fn color(class: &str, text: &str) -> Html {
    html! {
        <span class={classes!["nes-text", class.to_string()]}>{text}</span>
    }
}

#[function_component(GameBoard)]
pub fn game_board() -> Html {
    let (gcx, _) = use_store::<GameCommandExecutor>();

    match gcx.current_state() {
        GameState::Init => html! {
            <>
                <div class={classes!["nes-container", "is-rounded", "game-announcement"]}>
                    <h2>{"Let's start!"}</h2>
                    <ul>
                        <li>{"Type "} {color("is-primary", "start")} {" to start playing, "}{color("is-primary", "restart")}{"/"}{color("is-primary", "reset")}{" to restart with different map."}</li>
                        <li>{"You can use mouse or type these commands to play:"}
                            <ul>
                                <li>{color("is-success", "sxx")}{" to step on a tile, replace xx with the tile coordinate, column go first."}</li>
                                <li>{color("is-success", "fxx")}{" to flag the tile, "}{color("is-success", "uxx")}{" to unflag."}</li>
                                <li>{color("is-success", "nxx")}{" to step to all the concealed neighbors of xx, works if xx tile already stepped, you lose if one of the neighbor tile conceal a bomb."}</li>
                            </ul>
                        </li>
                    </ul>
                </div>
                <CommandInputForm />
            </>
        },
        _ => html! {
            <>
                <Board />
                <CommandInputForm />
            </>
        }
    }
}

#[function_component(ColumnLabel)]
fn column_label() -> Html {
    html! {
        <tr class={classes!["mines-column-label"]}>
            <th class={classes!["mine-cell"]}>{""}</th>
            <th class={classes!["mine-cell"]}>{"a"}</th>
            <th class={classes!["mine-cell"]}>{"b"}</th>
            <th class={classes!["mine-cell"]}>{"c"}</th>
            <th class={classes!["mine-cell"]}>{"d"}</th>
            <th class={classes!["mine-cell"]}>{"e"}</th>
            <th class={classes!["mine-cell"]}>{"f"}</th>
            <th class={classes!["mine-cell"]}>{"g"}</th>
            <th class={classes!["mine-cell"]}>{"h"}</th>
        </tr>
    }
}

#[function_component(Board)]
fn draw_board() -> Html {
    let (_, dispatch) = use_store::<GameStore>();
    let (hq, _) = use_store::<GameCommandExecutor>();
    let mut btn_classes = vec!["nes-btn"];
    let callback = dispatch.reduce_mut_callback_with(|store, ev: MouseEvent| {
        let btn_nth = ev.button();
        let button = ev.target_unchecked_into::<HtmlElement>();
        let cmd = if btn_nth == 0 { "s" } else { "t" };
        let x = button.get_attribute("data-x").unwrap();
        let y = button.get_attribute("data-y").unwrap();

        store
            .parse_command(&format!("{}{}{}", cmd, x, y))
            .unwrap_or_else(|err| store.errors.push(err));

        ev.prevent_default();
    });

    if *hq.current_state() == GameState::Lose || *hq.current_state() == GameState::Win {
        btn_classes.push("is-disabled");
    }

    let items = hq.board_map.iter().enumerate().map(|(y, row)| {
        html! {
            <tr> 
            <td class={classes!["mines-row-label"]}>{y+1}</td>
            { for row.iter().enumerate().map(|(x, cell)| html! {
                <td class={classes!["mine-cell"]}> {
                    if cell.clone() == TileState::Concealed {
                        html! {
                            <button
                                class={btn_classes.clone()}
                                onclick={callback.clone()}
                                oncontextmenu={callback.clone()}
                                data-x={(x+1).to_string()}
                                data-y={(y+1).to_string()} >
                            </button>
                        }
                    } else { html! {
                        <div data-x={(x+1).to_string()} data-y={(y+1).to_string()} oncontextmenu={callback.clone()} > {
                            if cell.clone() == TileState::Flagged { "🚩".to_string() }
                            else if cell.clone() == TileState::Revealed { "💣".to_string() }
                            else if cell.clone() == TileState::Detonated { "💥".to_string() }
                            else if hq.mines_map[y][x] == 0 { "".to_string() }
                            else { hq.mines_map[y][x].to_string() }
                        } </div>
                    }}
                } </td>
            }) } </tr>
        }
    });

    html! {
        <div class="nes-table-responsive">
            <table class={classes!["mines-field", "nes-table", "is-bordered", "is-centered"]}>
                <ColumnLabel />
                <tbody>{ items.collect::<Html>() }</tbody>
           </table>
        </div>
    }
}
