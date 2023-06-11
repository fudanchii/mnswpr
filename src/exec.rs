use rand::Rng;
use serde_wasm_bindgen::to_value;
use yewdux::prelude::*;

use crate::{
    external_binding::log,
    store::{GameCommand, GameState, GameStore},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum TileState {
    #[default]
    Closed,
    Flagged,
    Stepped,
    Detonated,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameCommandExecutor {
    pub mines_map: Vec<Vec<i8>>,
    pub board_map: Vec<Vec<TileState>>,
}

impl Listener for GameCommandExecutor {
    type Store = GameStore;

    fn on_change(&mut self, store: std::rc::Rc<Self::Store>) {
        let dispatch = Dispatch::<Self>::new();
        dispatch.apply(|_state| -> std::rc::Rc<_> {
            match store.game_state() {
                GameState::Init => self.init(),
                GameState::DrawBoard => match store.cmd() {
                    GameCommand::None => {}
                    GameCommand::Step(x, y) => self.step(*x, *y),
                    GameCommand::Flag(x, y) => self.flag(*x, *y),
                    GameCommand::Unflag(x, y) => self.unflag(*x, *y),
                },
                _ => {}
            }
            self.clone().into()
        });
    }
}

impl Store for GameCommandExecutor {
    fn new() -> Self {
        let mut slf = Self {
            mines_map: Vec::new(),
            board_map: Vec::new(),
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
        self.create_board_map();
        self.generate_mines_map();
    }
    fn create_board_map(&mut self) {
        for i in 0..8 {
            self.board_map.push(Vec::new());
            for _ in 0..8 {
                self.board_map[i].push(TileState::Closed);
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
            let idx: i8 = rng.gen_range(0..64);
            self.mines_map[(idx / 8) as usize][(idx % 8) as usize] = 99;
        }

        for i in 0..8 {
            for j in 0..8 {
                if self.mines_map[i][j] == 99 {
                    continue;
                }

                let mut counter = 0;
                for neighbour in self.neighbours(i, j).iter() {
                    if self.mines_map[neighbour.0][neighbour.1] == 99 {
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
            .iter()
            .filter(|(x, y)| *x >= 0 && *x < 8 && *y >= 0 && *y < 8)
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect()
    }

    fn open(&mut self, x: usize, y: usize) {
        log(to_value(&format!("open {} {}", x, y)).unwrap());
        self.board_map[x][y] = TileState::Stepped;
        if self.mines_map[x][y] > 0 {
            return;
        }
        for neighbour in self.neighbours(x, y) {
            if self.board_map[neighbour.0][neighbour.1] != TileState::Closed {
                continue;
            }
            if self.mines_map[neighbour.0][neighbour.1] == 0 {
                self.board_map[neighbour.0][neighbour.1] = TileState::Stepped;
                return self.open(neighbour.0, neighbour.1);
            }
            if self.mines_map[neighbour.0][neighbour.1] < 99 {
                self.board_map[neighbour.0][neighbour.1] = TileState::Stepped;
            }
        }
    }

    fn detonate(&mut self) {
        for i in 0..self.mines_map.len() {
            for j in 0..self.mines_map[i].len() {
                if self.mines_map[i][j] == 99 {
                    self.board_map[i][j] = TileState::Detonated;
                }
            }
        }
    }

    fn step(&mut self, x: usize, y: usize) {
        if self.mines_map[x][y] == 99 {
            self.detonate();
        }
        self.open(x, y);
    }

    fn flag(&mut self, x: usize, y: usize) {
        self.board_map[x][y] = TileState::Flagged;
    }

    fn unflag(&mut self, x: usize, y: usize) {
        self.board_map[x][y] = TileState::Closed;
    }
}
