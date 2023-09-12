use std::rc::Rc;

use lobars::raf::{use_request_animation_frame, RAFNext};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    current_seconds,
    exec::{GameCommandExecutor, GameState, TimerState},
};

pub const TIME_LIMIT: u64 = 60 * 5;

#[function_component(TimerDisplay)]
pub fn timer_display() -> Html {
    let (gcx, dispatch) = use_store::<GameCommandExecutor>();
    let clock = use_state(|| TIME_LIMIT);
    let raf = use_request_animation_frame();
    let display_clock = clock.clone();

    let display_class = match *clock {
        _ if gcx.timer_state == TimerState::Reset => "is-disabled",
        val if val <= (TIME_LIMIT / 4) => "is-error",
        val if val <= (TIME_LIMIT / 2) => "is-warning",
        _ => "is-primary",
    };

    {
        let gcx_dep = gcx.clone();
        use_effect_with_deps(
            move |_| {
                if &GameState::Init != gcx.current_state() {
                    raf.each(move |_| {
                        let started_at = match gcx.timer_state {
                            TimerState::Started(ts) => ts,
                            _ => return RAFNext::Abort,
                        };

                        let delta = current_seconds().saturating_sub(started_at);

                        let elapsed = TIME_LIMIT.saturating_sub(delta);
                        if elapsed != *clock {
                            clock.set(elapsed);
                        }

                        if elapsed == 0 {
                            dispatch.apply(|cgcx: Rc<GameCommandExecutor>| {
                                let mut new_gcx = (*cgcx).clone();
                                new_gcx.timer_checkin(elapsed);
                                new_gcx.into()
                            });
                            return RAFNext::Abort;
                        }

                        RAFNext::Continue
                    });
                }

                move || drop(raf)
            },
            gcx_dep.timer_state.clone(),
        );
    }

    html! {
        <button type="button" class={classes!["nes-btn", display_class]}>
            {m_ss(*display_clock)}
        </button>
    }
}

fn m_ss(time: u64) -> String {
    format!("{}:{:02}", time / 60, time % 60)
}
