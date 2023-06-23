use std::{collections::HashMap, rc::Rc};

use rand::Rng;
use yewdux::prelude::*;

use crate::{
    store::{GameCommand, GameState, GameStore},
    current_seconds,
};

const THE_BOMB: i8 = 99;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum TileState {
    #[default]
    Concealed,
    Flagged,
    Stepped,
    Detonated,
    Revealed,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TimerState {
    #[default]
    Reset,
    Started(u64),
    Paused,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameCommandExecutor {
    pub mines_map: Vec<Vec<i8>>,
    pub board_map: Vec<Vec<TileState>>,
    pub timer_state: TimerState,
    state: GameState,
    state_map: HashMap<GameState, Vec<GameState>>,
}

impl Listener for GameCommandExecutor {
    type Store = GameStore;

    fn on_change(&mut self, store: Rc<Self::Store>) {
        Dispatch::<Self>::new().apply(|state: Rc<GameCommandExecutor>| -> Rc<_> {
            let mut slf = (*state).clone();
            match slf.state {
                GameState::Init => (),
                GameState::Reinit => {
                    slf.init();
                    slf.transition_into(GameState::DrawBoard);
                }
                GameState::DrawBoard => {
                    slf.exec(store.cmd());
                }
                _ => {}
            }
            slf.into()
        });
    }
}

impl Store for GameCommandExecutor {
    fn new() -> Self {
        let mut slf = Self {
            mines_map: Vec::new(),
            board_map: Vec::new(),
            state: GameState::Init,
            state_map: Self::create_state_map(),
            timer_state: TimerState::Reset,
        };

        slf.init();

        slf
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl GameCommandExecutor {
    fn init(&mut self) {
        self.mines_map = Vec::new();
        self.board_map = Vec::new();
        self.timer_state = TimerState::Started(current_seconds());
        self.create_board_map();
        self.generate_mines_map();
    }

    fn exec(&mut self, cmd: &GameCommand) {
        match cmd {
            GameCommand::None => {}
            GameCommand::Step(x, y) => self.step(*x, *y),
            GameCommand::NeighboursStep(x, y) => self.neighbours_step(*x, *y),
            GameCommand::Flag(x, y) => self.flag(*x, *y),
            GameCommand::Unflag(x, y) => self.unflag(*x, *y),
            GameCommand::Toggle(x, y) => self.toggle_flag(*x, *y),
        }
    }

    fn create_state_map() -> HashMap<GameState, Vec<GameState>> {
        let mut states: HashMap<GameState, Vec<GameState>> = HashMap::new();
        states.insert(GameState::Init, vec![GameState::Reinit]);
        states.insert(
            GameState::DrawBoard,
            vec![
                GameState::Reinit,
                GameState::DrawBoard,
                GameState::Win,
                GameState::Lose,
            ],
        );
        states.insert(GameState::Reinit, vec![GameState::DrawBoard]);
        states.insert(GameState::Win, vec![GameState::Reinit]);
        states.insert(GameState::Lose, vec![GameState::Reinit]);
        states
    }

    pub fn timer_checkin(&mut self, current: u64) {
        if current == 0 {
            self.detonate_all();
        }
    }

    pub fn current_state(&self) -> &GameState {
        &self.state
    }

    pub fn transition_into(&mut self, state: GameState) {
        if self.state_map[&self.state].iter().any(|s| s == &state) {
            self.state = state;
        }
    }

    fn create_board_map(&mut self) {
        for i in 0..8 {
            self.board_map.push(Vec::new());
            for _ in 0..8 {
                self.board_map[i].push(TileState::Concealed);
            }
        }
    }

    fn generate_mines_map(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..8 {
            self.mines_map.push(Vec::new());
            for _ in 0..8 {
                self.mines_map[i].push(0);
            }
        }

        for _ in 0..16 {
            loop {
                let idx: i8 = rng.gen_range(0..64);
                if self.mines_map[(idx / 8) as usize][(idx % 8) as usize] == THE_BOMB {
                    continue;
                }
                self.mines_map[(idx / 8) as usize][(idx % 8) as usize] = THE_BOMB;
                break;
            }
        }

        for i in 0..8 {
            for j in 0..8 {
                if self.mines_map[i][j] == THE_BOMB {
                    continue;
                }

                let mut counter = 0;
                for neighbour in self.neighbours(i, j).iter() {
                    if self.mines_map[neighbour.0][neighbour.1] == THE_BOMB {
                        counter += 1;
                    }
                }
                self.mines_map[i][j] = counter;
            }
        }
    }

    fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let neighbours: Vec<(i8, i8)> = vec![
            (i as i8 - 1, j as i8 - 1),
            (i as i8 - 1, j as i8),
            (i as i8 - 1, j as i8 + 1),
            (i as i8, j as i8 - 1),
            (i as i8, j as i8 + 1),
            (i as i8 + 1, j as i8 - 1),
            (i as i8 + 1, j as i8),
            (i as i8 + 1, j as i8 + 1),
        ];
        neighbours
            .into_iter()
            .filter(|(x, y)| *x >= 0 && *x < 8 && *y >= 0 && *y < 8)
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }

    fn open(&mut self, x: usize, y: usize) {
        if self.mines_map[x][y] > 0 {
            return;
        }
        for neighbour in self.neighbours(x, y) {
            if self.board_map[neighbour.0][neighbour.1] != TileState::Concealed {
                continue;
            }

            // if center tile is 0, then no bomb in the neighbours, safe to step
            self.board_map[neighbour.0][neighbour.1] = TileState::Stepped;

            // if current neighbour is 0, recursively open the surrounding tiles for the neighbour
            if self.mines_map[neighbour.0][neighbour.1] == 0 {
                self.open(neighbour.0, neighbour.1);
            }
        }
    }

    fn detonate_all(&mut self) {
        self.all_bombs(99, 99, TileState::Detonated);
    }

    fn all_bombs(&mut self, x: usize, y: usize, state: TileState) {
        for i in 0..self.mines_map.len() {
            for j in 0..self.mines_map[i].len() {
                if x == i && y == j {
                    continue;
                }
                if self.mines_map[i][j] == THE_BOMB {
                    self.board_map[i][j] = state.clone();
                }
            }
        }
        self.timer_state = TimerState::Reset;
        self.transition_into(GameState::Lose);
    }

    fn step(&mut self, x: usize, y: usize) {
        if self.board_map[x][y] == TileState::Concealed {
            if self.mines_map[x][y] == THE_BOMB {
                self.board_map[x][y] = TileState::Detonated;
                return self.all_bombs(x, y, TileState::Revealed);
            }
            self.board_map[x][y] = TileState::Stepped;
            self.open(x, y);
        }
        if self.considered_win() {
            self.state = GameState::Win;
        }
    }

    fn neighbours_step(&mut self, x: usize, y: usize) {
        if self.board_map[x][y] == TileState::Concealed {
            return;
        }
        let neighbours = self.neighbours(x, y);
        let mut i: usize = 0;
        while self.state == GameState::DrawBoard {
            if i == neighbours.len() {
                break;
            }

            let n = neighbours[i];

            i += 1;

            if self.board_map[n.0][n.1] != TileState::Concealed {
                continue;
            }

            self.step(n.0, n.1);
        }
    }

    fn flag(&mut self, x: usize, y: usize) {
        if self.board_map[x][y] == TileState::Concealed {
            self.board_map[x][y] = TileState::Flagged;
        }
    }

    fn unflag(&mut self, x: usize, y: usize) {
        if self.board_map[x][y] == TileState::Flagged {
            self.board_map[x][y] = TileState::Concealed;
        }
    }

    fn toggle_flag(&mut self, x: usize, y: usize) {
        let tile = self.board_map[x][y].clone();
        if tile == TileState::Flagged {
            self.board_map[x][y] = TileState::Concealed;
        } else if tile == TileState::Concealed {
            self.board_map[x][y] = TileState::Flagged;
        }
    }

    // winning condition
    fn considered_win(&self) -> bool {
        self.all_closed_or_flagged_tiles_are_mines() && self.still_playing()
    }

    fn all_closed_or_flagged_tiles_are_mines(&self) -> bool {
        self.board_map
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, cell)| ((i, j), cell)))
            .filter(|(_, cell)| **cell == TileState::Concealed || **cell == TileState::Flagged)
            .all(|((i, j), _)| self.mines_map[i][j] == THE_BOMB)
    }

    fn still_playing(&self) -> bool {
        self.state != GameState::Lose
    }
}
