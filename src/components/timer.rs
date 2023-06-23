use std::{rc::Rc, cell::RefCell};

use gloo_render::AnimationFrame;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    exec::{GameCommandExecutor, TimerState},
    store::GameState,
    current_seconds
};

pub const TIME_LIMIT: u64 = 60 * 3;

fn raf_callback(rafcell: Rc<RefCell<Option<AnimationFrame>>>, cb: Box<dyn Fn() -> bool>) {
    let rafcell_clone = rafcell.clone();
    if !cb() {
        *rafcell.borrow_mut() = None;
        return;
    }
     *rafcell.borrow_mut() = Some(gloo_render::request_animation_frame(move |_| {
        raf_callback(rafcell_clone, Box::new(move || cb()));
    }));
}

#[function_component(TimerDisplay)]
pub fn timer_display() -> Html {
    let (gcx, dispatch) = use_store::<GameCommandExecutor>();
    let clock = use_state(|| TIME_LIMIT);
    let raf = use_mut_ref(|| None);
    let display_clock = clock.clone();

    let display_class = match *clock {
        _ if gcx.timer_state == TimerState::Reset => "is-disabled",
        val if val <= (TIME_LIMIT / 4) => "is-error",
        val if val <= (TIME_LIMIT / 2) => "is-warning",
        _ => "is-primary",
    };

    {
        let gcx = gcx.clone();
        let gcx_dep = gcx.clone();
        let raf = raf.clone();
        let raf_clone = raf.clone();
        use_effect_with_deps(move |_| {
            if gcx.current_state() == &GameState::DrawBoard {
                if let TimerState::Started(started_at) = gcx.timer_state {
                    *raf.borrow_mut() = Some(gloo_render::request_animation_frame(move |_| {
                        raf_callback(raf_clone, Box::new(move || {
                            let delta = current_seconds().checked_sub(started_at).unwrap_or(0);
                            let elapsed = TIME_LIMIT.checked_sub(delta).unwrap_or(0);
                            if elapsed != *clock {
                                clock.set(elapsed);
                            }
                            if elapsed == 0 {
                                dispatch.apply(|cgcx: Rc<GameCommandExecutor>| {
                                    let mut new_gcx = (*cgcx).clone();
                                    new_gcx.timer_checkin(elapsed);
                                    new_gcx.into()
                                });
                                return false;
                            }
                            true
                        }));
                    }));
                }
            }

            move || {
                *raf.borrow_mut() = None;
            }
        }, gcx_dep.timer_state.clone());
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