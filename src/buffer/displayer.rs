use super::buffer::Buffer;

use std::io;
use std::io::Write;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Stylize;

impl Buffer {
    pub fn display_command(&self, command: &String) {
        self.super_display();
        execute!(io::stdout(), MoveTo(0, self.max_y as u16)).unwrap();
        print!("{}", command);
        io::stdout().flush().unwrap();
    }
    pub fn super_display(&self) {
        execute!(
            io::stdout(),
            MoveTo((self.max_x - 30) as u16, self.max_y as u16)
        )
        .unwrap();
        self.pointer_pos.display();
        print!("line len : {}", self.container[self.pointer_pos.y].len());
        for (y, row) in self.container.iter().enumerate() {
            execute!(io::stdout(), MoveTo(0, y as u16)).unwrap();
            for (x, cell) in row.iter().enumerate() {
                if self.pointer_pos.x == x && self.pointer_pos.y == y {
                    print!("{}{}", cell.blue(), "|".red());
                } else if *cell == '\n' {
                    print!(" ");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
        println!();
        io::stdout().flush().unwrap();
    }
}
