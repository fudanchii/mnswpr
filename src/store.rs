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
        if let Some(prev_cmd) = self.cmd_history.last() {
            if prev_cmd == cmd {
                return Ok(());
            }
        }
        self.current_cmd = GameCommand::None;
        if let Some((prefix, args)) = cmd.split_once(' ') {
            self.current_cmd = (prefix, args).try_into()?;
        } else {
            match cmd {
                "restart" => {
                    self.transition_into(GameState::Reinit);
                    self.cmd_history.clear();
                }
                "start" => self.transition_into(GameState::DrawBoard),
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
    Sys(String),
    Step(usize, usize),
    Flag(usize, usize),
    Unflag(usize, usize),
    Toggle(usize, usize),
}

impl TryFrom<(&str, &str)> for GameCommand {
    type Error = GameError;

    fn try_from(this: (&str, &str)) -> Result<Self, Self::Error> {
        let (cmd, args) = this;
        let (x, y) = parse_coordinate(args)?;
        let cmd: &str = &cmd.to_lowercase();
        match cmd {
            "" => Ok(GameCommand::None),
            "go" | "goto" | "g" | "step" | "s" => Ok(GameCommand::Step(x, y)),
            "flag" | "f" | "mark" | "m" => Ok(GameCommand::Flag(x, y)),
            "unflag" | "u" | "unmark" => Ok(GameCommand::Unflag(x, y)),
            "toggle" | "t" => Ok(GameCommand::Toggle(x, y)),
            _ => Err(GameError::UnknownCommand),
        }
    }
}

fn parse_coordinate(val: &str) -> Result<(usize, usize), GameError> {
    let mut arg = val.chars();
    let i = arg
        .next()
        .ok_or(GameError::InvalidArgument)
        .and_then(|num| num.to_digit(10).ok_or(GameError::InvalidArgument))?;
    let j = arg
        .next()
        .ok_or(GameError::InvalidArgument)
        .and_then(|num| num.to_digit(10).ok_or(GameError::InvalidArgument))?;
    Ok((j as usize - 1, i as usize - 1))
}
