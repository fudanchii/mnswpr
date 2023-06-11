use std::collections::HashMap;

use serde_wasm_bindgen::to_value;
use yew::platform::spawn_local;
use yewdux::prelude::*;

use crate::errors::GameError;
use crate::exec::GameCommandExecutor;
use crate::external_binding::invoke;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameStore {
    current_state: GameState,
    current_cmd: GameCommand,
    state_map: HashMap<GameState, Vec<GameState>>,
    pub errors: Vec<GameError>,
}

impl Store for GameStore {
    fn new() -> Self {
        let mut slf = Self::default();
        init_listener(GameCommandExecutor::new());
        slf.create_state_map();
        slf
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl GameStore {
    pub fn parse_command(&mut self, cmd: &str) -> Result<(), GameError> {
        self.current_cmd = GameCommand::None;
        if let Some((prefix, args)) = cmd.split_once(' ') {
            self.current_cmd = (prefix, args).try_into()?;
        } else {
            match cmd {
                "cancel" => self.transition_into(GameState::Init),
                "start" => self.transition_into(GameState::DrawBoard),
                "quit" | "exit" => spawn_local(async {
                    invoke("exit", to_value(&()).unwrap()).await;
                }),
                _ => return Err(GameError::UnknownCommand),
            }
        }
        Ok(())
    }

    pub fn cmd(&self) -> &GameCommand {
        &self.current_cmd
    }

    pub fn game_state(&self) -> GameState {
        self.current_state.clone()
    }

    pub fn transition_into(&mut self, state: GameState) {
        if self.state_map[&self.current_state]
            .iter()
            .any(|x| x == &state)
        {
            self.current_state = state;
        }
    }

    fn create_state_map(&mut self) {
        let mut states: HashMap<GameState, Vec<GameState>> = HashMap::new();
        states.insert(GameState::Init, vec![GameState::DrawBoard]);
        states.insert(
            GameState::DrawBoard,
            vec![
                GameState::Init,
                GameState::DrawBoard,
                GameState::Win,
                GameState::Lose,
            ],
        );
        states.insert(GameState::Win, vec![GameState::Init]);
        states.insert(GameState::Lose, vec![GameState::Init]);
        self.state_map = states;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Init,
    DrawBoard,
    Win,
    Lose,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameCommand {
    #[default]
    None,
    Step(usize, usize),
    Flag(usize, usize),
    Unflag(usize, usize),
}

impl TryFrom<(&str, &str)> for GameCommand {
    type Error = GameError;

    fn try_from(this: (&str, &str)) -> Result<Self, Self::Error> {
        let (cmd, args) = this;
        let (x, y) = parse_coordinate(args)?;
        match cmd {
            "" => Ok(GameCommand::None),
            "go" | "goto" | "g" | "step" | "s" | "touch" | "t" => Ok(GameCommand::Step(x, y)),
            "flag" | "f" | "mark" | "m" => Ok(GameCommand::Flag(x, y)),
            "unflag" | "u" | "unmark" => Ok(GameCommand::Unflag(x, y)),
            _ => Err(GameError::UnknownCommand),
        }
    }
}

fn parse_coordinate(val: &str) -> Result<(usize, usize), GameError> {
    let mut arg = val.chars();
    let x = arg
        .next()
        .ok_or(GameError::InvalidArgument)
        .and_then(|num| num.to_digit(10).ok_or(GameError::InvalidArgument))?;
    let y = arg
        .next()
        .ok_or(GameError::InvalidArgument)
        .and_then(|num| num.to_digit(10).ok_or(GameError::InvalidArgument))?;
    Ok((x as usize, y as usize))
}
