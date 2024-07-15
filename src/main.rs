use buffer::buffer_mod::Buffer;
use crossterm::terminal::size;

mod buffer;

fn main() {
    match size() {
        Ok((x, y)) => {
            let mut buffer = Buffer::init(x as usize, (y - 3) as usize);
            if let Err(e) = buffer.listen() {
                eprintln!("error : {}", e);
            }
        }
        Err(e) => {
            eprintln!("error : {}", e);
            Buffer::init(10, 10);
        }
    }
}
