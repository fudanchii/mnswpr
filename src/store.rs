use std::rc::Rc;

use serde_wasm_bindgen::to_value;
use yew::platform::spawn_local;
use yewdux::prelude::*;

use crate::errors::GameError;
use crate::exec::GameCommandExecutor;
use crate::external_binding::{invoke, log};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameStore {
    pub current_cmd: GameCommand,
    pub errors: Vec<GameError>,
    pub cmd_history: Vec<String>,
}

impl Store for GameStore {
    fn new() -> Self {
        let slf = Self::default();
        init_listener(GameCommandExecutor::new());
        slf
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl GameStore {
    pub fn parse_command(&mut self, cmd: &str) -> Result<(), GameError> {
        let cmd: &str = &cmd.to_lowercase();
        self.current_cmd = GameCommand::None;
        let cmds: Vec<char> = cmd.chars().collect();
        if cmds.len() == 3 {
            let mut act_cmds: [char; 3] = [' '; 3];
            act_cmds.iter_mut().enumerate()
                .for_each(|(i, v)| *v = cmds[i]);
            self.current_cmd = act_cmds.try_into()?;
        } else {
            match cmd {
                "restart" | "reset" => self.transition_into(GameState::Reinit),
                "start" => {
                    if self.cmd_history.is_empty() {
                        self.transition_into(GameState::Reinit);
                    }
                },
                "quit" | "exit" => spawn_local(async {
                    invoke("exit", to_value(&()).unwrap()).await;
                }),
                _ => {
                    self.cmd_history.push(cmd.to_string());
                    return Err(GameError::UnknownCommand);
                }
            }
        }
        self.cmd_history.push(cmd.to_string());
        Ok(())
    }

    pub fn cmd(&self) -> &GameCommand {
        &self.current_cmd
    }

    pub fn transition_into(&self, state: GameState) {
        let dispatch = Dispatch::<GameCommandExecutor>::new();
        log(format!("transition  {:?}", state).into());
        dispatch.apply(|gcx: Rc<GameCommandExecutor>| {
            let mut gcx = (*gcx).clone();
            gcx.transition_into(state);
            gcx.into()
        });
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Init,
    Reinit,
    DrawBoard,
    Win,
    Lose,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameCommand {
    #[default]
    None,
    Step(usize, usize),
    NeighboursStep(usize, usize),
    Flag(usize, usize),
    Unflag(usize, usize),
    Toggle(usize, usize),
}

impl TryFrom<[char; 3]> for GameCommand {
    type Error = GameError;

    fn try_from(this: [char; 3]) -> Result<Self, Self::Error> {
        let [cmd, c1, c2] = this;
        let i = match c1 {
            'a'..='z' => c1 as usize - 'a' as usize,
            '1'..='9' => c1 as usize - '1' as usize,
            _ => return Err(GameError::InvalidArgument),
        };
        let j = match c2 {
            '1'..='9' => c2 as usize - '1' as usize,
            _ => return Err(GameError::InvalidArgument),
        };
        match cmd {
            's' => Ok(GameCommand::Step(j, i)),
            'f' => Ok(GameCommand::Flag(j, i)),
            'u' => Ok(GameCommand::Unflag(j, i)),
            't' => Ok(GameCommand::Toggle(j, i)),
            'n' => Ok(GameCommand::NeighboursStep(j, i)),
            _ => Err(GameError::UnknownCommand),
        }
    }
}