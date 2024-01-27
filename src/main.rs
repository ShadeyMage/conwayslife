use std::{
    io::{self, Write},
    time::{self, SystemTime, UNIX_EPOCH}
};

use spin_sleep;

use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use serde::{Serialize, Deserialize};
use clap::Parser;

const CLEAR_SCR: &str = "[2J";
const FIRST_COL: &str = "[1;1H";
const PAD: usize = 1;

const BORDER: char = '▒';
const BLOCK: char = '█';
const BLANK: char = ' ';
const RAND_THRESHOLD: u8 = u8::MAX / 6;

use crate::CellState::{Alive, Border, Dead};

macro_rules! flush {
    () => {
        io::stdout().flush().unwrap();
    }
}

macro_rules! clear_screen {
    () => {
        print!("{esc}{CS}{esc}{FC}", esc = 27 as char, CS = CLEAR_SCR, FC = FIRST_COL);
    };
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 50)]
    width: usize,
    #[arg(long, default_value_t = 50)]
    height: usize,
    #[arg(long, short, default_value_t = 0)]
    seed: u64,
    #[arg(long, short, default_value_t= 500)]
    time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum CellState {
    Alive,
    Border,
    Dead,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Cell {
    state: CellState,
    will_live: bool,
    symbol: char,
}
impl Cell {
    fn new(state: CellState) -> Self {
        Self {
            state,
            will_live: false,
            symbol: match state {
                Alive => BLOCK,
                Border => BORDER,
                Dead => BLANK,
            }
        }
    }

    fn set_state(&mut self, state: CellState) {
        self.state = state;
        self.symbol = match state {
            Alive => BLOCK,
            Border => BORDER,
            Dead => BLANK,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Board {
    spaces: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}
impl Board {
    fn new(width: usize, height: usize) -> Self {
        
        let mut cells: Vec<Vec<Cell>> = Vec::new();

        cells.push(vec![Cell::new(Border); width + (PAD * 2)]);

        for _ in 0..height {
            cells.push(pad(vec![Cell::new(Dead); width]))
        }
        cells.push(vec![Cell::new(Border); width + (PAD * 2)]);
        Board {
            spaces: cells,
            width,
            height,
        }
    }

    fn randomize_rows(&mut self, seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        for row in self.spaces.iter_mut() {
            for cell in row.iter_mut() {
                match cell.state {
                    Border => {},
                    _ => {
                        let value: u8 = rng.gen::<u8>();
                        if value < RAND_THRESHOLD {
                            cell.set_state(Alive);
                        }
                    }
                }
            }
        }
    }

    fn update_will(&mut self) {
        for (horizontal_index, row) in self.clone().spaces.iter_mut().enumerate() {
            for (vertical_index, cell) in row.clone().iter_mut().enumerate() {
                match cell.state {
                    Alive => {
                        let neighbors = self.neighbour_check(horizontal_index, vertical_index);
                        match neighbors {
                            2 | 3 => self.spaces[horizontal_index][vertical_index].will_live = true,
                            _ => self.spaces[horizontal_index][vertical_index].will_live = false,
                        }
                    },
                    Border => {},
                    Dead => {
                        let neighbors = self.neighbour_check(horizontal_index, vertical_index);
                        match neighbors {
                            3 => self.spaces[horizontal_index][vertical_index].will_live = true,
                            _ => self.spaces[horizontal_index][vertical_index].will_live = false
                        }
                    }
                }
            }
        }
    }

    fn change_based_on_will(&mut self) {
        for row in self.spaces.iter_mut() {
            for cell in row.iter_mut() {
                match (cell.state, cell.will_live) {
                    (Border,_) => {},
                    (Alive | Dead, true) => cell.set_state(Alive),
                    (_,_) => cell.set_state(Dead)
                }
                cell.will_live = false;
            }
        }
    }

    fn neighbour_check(&self, row: usize, col: usize) -> usize {
        // because of padding, assume always inbound
        let chk: usize = vec![
            // check horizontal neighbours
            self.spaces[row][col+1], self.spaces[row][col-1],
            // check diagonal neighbours
            self.spaces[row-1][col-1], self.spaces[row+1][col+1], self.spaces[row+1][col-1], self.spaces[row-1][col+1],
            // check vertical neighbours
            self.spaces[row-1][col],self.spaces[row + 1][col]
        ].iter().filter(|&cell| matches!(cell.state, Alive)).count();
        chk
    }

    fn reveal(&self, frame: u128) {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let mut res = String::new();
        for row in &self.spaces {
            for cell in row {
                res.push(cell.symbol)
            }
            res.push('\n');
        }
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("{}\nFrames: {}\nTime to Print: {:?}\r\n", res, frame, (end-start));
        flush!();
    }

    fn game_loop(&mut self, delay: time::Duration) {
        let mut frame_counter: u128 = 0;
        loop {
            frame_counter += 1;
            clear_screen!();
            self.update_will();
            self.change_based_on_will();
            self.reveal(frame_counter);
            spin_sleep::sleep(delay);
        }
    }
}

fn pad(mut v: Vec<Cell>) -> Vec<Cell> {
    v.insert(0, Cell::new(Border));
    v.push(Cell::new(Border));
    v
}

fn main() {
    let mut rng = rand::thread_rng();
    let args: Args = Args::parse();
    let seed_val = match args.seed {
        0 => rng.gen::<u64>(),
        _ => args.seed,
    };

    let mut test: Board = Board::new(
        args.width, 
        args.height,
    );
    test.randomize_rows(seed_val);
    clear_screen!();
    test.reveal(1);
    
    test.game_loop(
        time::Duration::from_millis(args.time)
    ); 
}
