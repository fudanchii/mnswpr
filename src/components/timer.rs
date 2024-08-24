use std::rc::Rc;

use lobars::raf::{use_request_animation_frame, RAFNext};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    current_seconds,
    exec::{GameCommandExecutor, GameState, TimerState, TIME_LIMIT},
};

#[function_component(TimerDisplay)]
pub fn timer_display() -> Html {
    let (gcx, dispatch) = use_store::<GameCommandExecutor>();
    let clock = use_state(|| TIME_LIMIT);
    let raf = use_request_animation_frame();
    let display_clock = clock.clone();

    let display_class = gcx.timer_display_class(
        &clock,
        "is-disabled",
        "is-warning",
        "is-danger",
        "is-primary",
    );

    let pause_callback = {
        let clock = clock.clone();
        dispatch.reduce_mut_callback(move |store| {
            store.timer_pause_toggle(*clock);
        })
    };

    {
        let gcx_dep = gcx.clone();
        use_effect_with(gcx_dep.timer_state.clone(), move |_| {
            if &GameState::Init != gcx.current_state() {
                raf.each(move |_| {
                    let started_at = match gcx.timer_state {
                        TimerState::Started(ts) => ts,
                        _ => return RAFNext::Abort,
                    };

                    let elapsed = current_seconds().saturating_sub(started_at);

                    let eta = gcx.time_left.saturating_sub(elapsed);
                    if eta != *clock {
                        clock.set(eta);
                    }

                    if eta == 0 {
                        dispatch.apply(|cgcx: Rc<GameCommandExecutor>| {
                            let mut new_gcx = (*cgcx).clone();
                            new_gcx.timer_checkin(eta);
                            new_gcx.into()
                        });
                        return RAFNext::Abort;
                    }

                    RAFNext::Continue
                });
            }

            move || drop(raf)
        });
    }

    html! {
        <button type="button" class={classes!["nes-btn", display_class]} onclick={pause_callback}>
            {m_ss(*display_clock)}
        </button>
    }
}

fn m_ss(time: u64) -> String {
    format!("{}:{:02}", time / 60, time % 60)
}
