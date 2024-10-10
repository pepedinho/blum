use super::popup::Popup;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::Mutex;
use std::{io, time::Duration};

use crossterm::execute;
use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal::{self, ClearType},
};

lazy_static! {
    static ref PREVIOUS_CHAR: Mutex<char> = Mutex::new(' ');
}

pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub struct Buffer {
    pub pointer_pos: Coord,
    pub max_x: usize,
    pub max_y: usize,
    pub lines_count: usize,
    pub filename: Vec<String>,
    pub container: Vec<Vec<char>>,
}

impl Coord {
    pub fn init() -> Coord {
        let x = 0;
        let y = 0;
        Coord { x, y }
    }
    pub fn display(&self) {
        print!("x: {} y: {}", self.x, self.y);
    }
}

impl Buffer {
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        for row in &self.container {
            for &cell in row {
                write!(file, "{}", cell)?;
            }
            writeln!(file)?;
        }
        Ok(())
    }
    pub fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        for (y, line) in reader.lines().enumerate() {
            let line = line?;
            let mut new_row = vec![' '; line.len()];
            for (x, char) in line.chars().enumerate() {
                if x < self.max_x && y < self.max_y {
                    new_row[x] = char;
                }
            }
            self.container.push(new_row);
            self.lines_count += 1;
        }
        Ok(())
    }
    pub fn init(x: usize, y: usize, filename: Vec<String>) -> Buffer {
        let pointer_pos = Coord::init();
        let container: Vec<Vec<char>> = vec![vec![' '; 1]; 1];
        let max_x = x;
        let max_y = y;
        let lines_count: usize = 0;
        Buffer {
            lines_count,
            filename,
            max_x,
            max_y,
            pointer_pos,
            container,
        }
    }
    pub fn listen(&mut self) -> io::Result<()> {
        let mut coord = Coord::init();
        execute!(io::stdout(), terminal::Clear(ClearType::All))?;
        terminal::enable_raw_mode()?;
        let mut command_mode = false;
        let mut command_buffer = String::new();
        if self.filename.len() > 1 {
            let filename = self.filename[1].clone();
            if Path::new(self.filename[1].as_str()).exists() {
                if let Err(e) = self.load_from_file(filename.as_str()) {
                    eprintln!("error during loading file : {}", e);
                }
            }
        }
        loop {
            if event::poll(Duration::from_millis(5))? {
                if let event::Event::Key(KeyEvent {
                    code,
                    modifiers: _,
                    state: _,
                    kind: _,
                }) = event::read()?
                {
                    if command_mode {
                        match code {
                            KeyCode::Enter => match command_buffer.as_str() {
                                ":q" => break,
                                ":w" => {
                                    if self.filename.len() > 1 {
                                        let filename = self.filename[1].clone();
                                        if let Err(e) = self.save_to_file(filename.as_str()) {
                                            eprintln!("error during saving : {}", e);
                                        }
                                    }
                                    command_mode = false;
                                }
                                ":wq" => {
                                    if self.filename.len() > 1 {
                                        let filename = self.filename[1].clone();
                                        if let Err(e) = self.save_to_file(filename.as_str()) {
                                            eprintln!("error during saving : {}", e);
                                        }
                                        break;
                                    } else {
                                        let mut pop =
                                            Popup::new("No filename", "your file is no name");
                                        pop.show(self.max_x, self.max_y);
                                        command_buffer.clear();
                                        if let Err(e) = pop.listen(&mut command_buffer, self) {
                                            eprintln!(
                                                "error during listening your keyboard: {}",
                                                e
                                            );
                                        }
                                        break;
                                    }
                                }
                                _ => {
                                    command_mode = false;
                                }
                            },
                            KeyCode::Esc => {
                                command_mode = false;
                                command_buffer.clear();
                            }
                            KeyCode::Char(c) => {
                                command_buffer.push(c);
                            }
                            KeyCode::Backspace => {
                                command_buffer.pop();
                            }
                            _ => {}
                        }
                        self.display_command(&command_buffer);
                    } else {
                        self.trigger(code, &mut command_buffer, &mut command_mode, &mut coord);
                        self.super_display();
                    }
                    //execute!(io::stdout(), terminal::Clear(ClearType::All))?;
                }
            }
        }
        terminal::disable_raw_mode()?;
        Ok(())
    }
    pub fn trigger(
        &mut self,
        code: KeyCode,
        command_buffer: &mut String,
        command_mode: &mut bool,
        coord: &mut Coord,
    ) {
        match code {
            KeyCode::Esc => {
                command_buffer.clear();
                *command_mode = true;
            }
            KeyCode::Up => {
                //self.destroy_pointer(&mut coord, 'd');
                self.moove_on(coord, 'u', '|');
            }
            KeyCode::Down => {
                //self.destroy_pointer(&mut coord, 'd');
                self.moove_on(coord, 'd', '|');
            }
            KeyCode::Left => {
                //self.destroy_pointer(&mut coord, 'l');
                self.moove_on(coord, 'l', '|');
            }
            KeyCode::Right => {
                //self.destroy_pointer( coord, 'd');
                self.moove_on(coord, 'r', '|');
            }
            KeyCode::Char(c) => {
                //self.put_on(self.pointer_pos.x, self.pointer_pos.y, c);
                self.write_on(coord, 'r', c);
                //self.write_on( coord, 'r', '|');
            }
            KeyCode::Backspace => {
                self.destroy_pointer(coord, 'l');
                self.moove_on(coord, 'l', '|');
            }
            KeyCode::Enter => {
                //self.destroy_pointer( coord, 'e');
                self.moove_on(coord, 'e', '|');
            }
            _ => {}
        }
    }
}
