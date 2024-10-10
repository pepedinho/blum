use std::env;
mod buffer;

use buffer::buffer::*;
use crossterm::terminal::size;

fn main() {
    let args: Vec<String> = env::args().collect();
    match size() {
        Ok((x, y)) => {
            let mut buffer = Buffer::init(x as usize, (y - 3) as usize, args);
            if let Err(e) = buffer.listen() {
                eprintln!("error : {}", e);
            }
        }
        Err(e) => {
            eprintln!("error : {}", e);
            Buffer::init(10, 10, args);
        }
    }
}
