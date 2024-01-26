use std::{
    io::{self, Write},
    thread, time,
};

use clap::Parser;

const FIRSTCOL: &str = "\x1b[H";
const CLEARSCR: &str = "\x1b[2J";

const ALIVE: &str = "█";
const DEAD: &str = " ";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 50)]
    width: u16,
    #[arg(short, long, default_value_t = 50)]
    height: u16,
}

#[derive(Debug)]
struct Cell {
    alive: bool,
    neighbors: u8,
}
impl Cell {
    fn update(&mut self) {}

    fn reveal(self) {
        match self.alive {
            true => print!("{}", ALIVE),
            false => print!("{}", DEAD),
        }
    }
}

#[derive(Debug)]
struct Board {
    cells: Vec<Cell>,
    width: u16,
    height: u16,
}
impl Board {}

fn update_screen() {
    match io::stdout().flush() {
        Ok(_) => print!("{}{}", FIRSTCOL, CLEARSCR),
        Err(_) => unimplemented!(),
    }
}

fn main() {
    let ten_seconds = time::Duration::from_secs(10);
    print!("Hello, world!");
    update_screen();
    thread::sleep(ten_seconds); // Print initial message // Move the cursor back to the beginning of the current line
    print!("\r\x1b[K"); // Clear from the cursor position to end of line
    println!("New message!"); // Print new message on the same line
}
