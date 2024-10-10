use super::buffer::{Buffer, Coord};

use std::{
    io::{self, Write},
    time::Duration,
    usize,
};

use crossterm::{
    cursor::MoveTo,
    event::{self, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};

pub struct Popup {
    pub title: String,
    pub message: String,
}

impl Popup {
    pub fn new(title: &str, message: &str) -> Self {
        Popup {
            title: title.to_string(),
            message: message.to_string(),
        }
    }

    pub fn show(&self, max_x: usize, max_y: usize) {
        let mut middle = Coord::init();
        middle.x = max_x / 2;
        middle.y = max_y / 2;

        let message_len = self.message.len();

        execute!(
            io::stdout(),
            MoveTo(
                (middle.x - message_len / 2 - 2) as u16,
                (middle.y - 1) as u16
            )
        )
        .unwrap();

        for _ in 0..(message_len + 4) {
            print!("-");
        }
        println!();

        execute!(
            io::stdout(),
            MoveTo((middle.x - message_len / 2 - 2) as u16, middle.y as u16)
        )
        .unwrap();

        println!("| {} |", self.message);

        execute!(
            io::stdout(),
            MoveTo(
                (middle.x - message_len / 2 - 2) as u16,
                (middle.y + 1) as u16
            )
        )
        .unwrap();

        for _ in 0..(message_len + 4) {
            print!("-");
        }
        println!();
    }
    pub fn display_command(&self, command: &String, buffer: &mut Buffer) {
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();
        buffer.super_display();
        execute!(io::stdout(), MoveTo(0, buffer.max_y as u16)).unwrap();
        print!("{}", command);
        io::stdout().flush().unwrap();
    }
    pub fn listen(&mut self, command_buffer: &mut String, buffer: &mut Buffer) -> io::Result<()> {
        loop {
            if event::poll(Duration::from_millis(5))? {
                if let event::Event::Key(KeyEvent {
                    code,
                    modifiers: _,
                    state: _,
                    kind: _,
                }) = event::read()?
                {
                    match code {
                        event::KeyCode::Char(c) => {
                            command_buffer.push(c);
                        }
                        event::KeyCode::Enter => {
                            buffer.filename.push(command_buffer.clone());
                            if let Err(e) = buffer.save_to_file(buffer.filename[1].as_str()) {
                                eprintln!("error during saving : {}", e);
                            }
                            break;
                        }
                        event::KeyCode::Backspace => {
                            command_buffer.pop();
                        }
                        event::KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                    self.display_command(command_buffer, buffer)
                }
            }
        }
        Ok(())
    }
}
