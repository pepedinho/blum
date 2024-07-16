pub mod buffer_mod {
    use lazy_static::lazy_static;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use std::path::Path;
    use std::sync::Mutex;
    use std::{io, time::Duration};

    use crossterm::cursor::MoveTo;
    use crossterm::execute;
    use crossterm::style::{Color, Stylize};
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
                for (x, char) in line.chars().enumerate() {
                    if x < self.max_x && y < self.max_y {
                        self.container[y].push(char);
                    }
                }
                self.container.push(vec![' ']);
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
        pub fn get_on_this_pos(&self, coord: &mut Coord) -> char {
            return self.container[coord.y][coord.x];
        }
        pub fn put_on(&mut self, x: usize, y: usize, elem: char) {
            if x < self.max_x as usize && y < self.max_y as usize {
                if y >= self.container.len() {
                    while y >= self.container.len() {
                        self.container.push(vec![' ']);
                    }
                }
                if x >= self.container[y].len() {
                    self.container[y].push(elem);
                }
                self.container[y][x] = elem;
            }
        }
        pub fn update_prev_char(&mut self, coord: &mut Coord, direction: char) {
            let mut previous_char = PREVIOUS_CHAR.lock().unwrap();
            if *previous_char != ' ' {
                match direction {
                    'u' => self.put_on(coord.x, coord.y + 1, *previous_char),
                    'd' => self.put_on(coord.x, coord.y - 1, *previous_char),
                    'r' => self.put_on(coord.x - 1, coord.y, *previous_char),
                    's' => self.put_on(coord.x, coord.y - 1, *previous_char),
                    'l' => self.put_on(coord.x + 1, coord.y, *previous_char),
                    'm' => self.put_on(self.max_x - 1, coord.y + 1, *previous_char),
                    _ => {}
                };
            }
            *previous_char = self.get_on_this_pos(coord);
        }
        pub fn moove_on(&mut self, coord: &mut Coord, direction: char, elem: char) {
            match direction {
                'u' => {
                    if coord.y > 0 {
                        coord.y -= 1;
                        if coord.x > self.container[coord.y].len() - 1 {
                            coord.x = self.container[coord.y].len() - 1;
                        }
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        if elem == '|' {
                            self.update_prev_char(coord, 'u');
                        }
                        self.put_on(coord.x as usize, (coord.y) as usize, elem);
                    }
                }
                'd' => {
                    if coord.y < self.lines_count {
                        coord.y += 1;
                        coord.x = self.container[coord.y].len() - 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        if elem == '|' {
                            self.update_prev_char(coord, 'd');
                        }
                        self.put_on(coord.x as usize, (coord.y) as usize, elem);
                    }
                }
                'r' => {
                    if coord.x < self.container[coord.y].len() {
                        coord.x += 1;
                        if elem == '|' && coord.x >= self.container[coord.y].len() {
                            return;
                        }
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        if elem == '|' {
                            self.update_prev_char(coord, 'r');
                        }
                        self.put_on((coord.x) as usize, coord.y as usize, elem);
                    } else if coord.x == self.max_x - 1 && coord.y < self.max_y - 1 {
                        coord.x = 0;
                        coord.y += 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        if elem == '|' {
                            self.update_prev_char(coord, 's');
                        }
                        self.put_on((coord.x) as usize, coord.y as usize, elem);
                    }
                }
                'l' => {
                    if coord.x > 1 {
                        coord.x -= 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        if elem == '|' {
                            self.update_prev_char(coord, 'l');
                        }
                        self.put_on(coord.x as usize, coord.y as usize, elem);
                    } else if coord.x == 0 && coord.y > 0 {
                        coord.x = self.max_x - 1;
                        coord.y -= 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        if elem == '|' {
                            self.update_prev_char(coord, 'm');
                        }
                        self.put_on(coord.x as usize, coord.y as usize, elem);
                    }
                }
                'e' => {
                    if coord.y < self.max_y - 1 {
                        coord.x = 0;
                        coord.y += 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        self.put_on((coord.x) as usize, coord.y as usize, elem);
                    }
                }
                _ => {}
            }
        }
        pub fn write_on(&mut self, coord: &mut Coord, direction: char, elem: char) {
            match direction {
                'u' => {
                    if coord.y > 0 {
                        coord.y -= 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        self.put_on(coord.x as usize, (coord.y) as usize, elem);
                    }
                }
                'd' => {
                    if coord.y < self.max_y - 1 {
                        coord.y += 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        self.put_on(coord.x as usize, (coord.y) as usize, elem);
                    }
                }
                'r' => {
                    if coord.x < self.max_x - 1 {
                        coord.x += 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        self.put_on((coord.x) as usize, coord.y as usize, elem);
                    } else if coord.x == self.max_x - 1 && coord.y < self.max_y - 1 {
                        coord.x = 0;
                        coord.y += 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        self.put_on((coord.x) as usize, coord.y as usize, elem);
                    }
                }
                'l' => {
                    if coord.x > 0 {
                        coord.x -= 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        self.put_on(coord.x as usize, coord.y as usize, elem);
                    } else if coord.x == 0 && coord.y > 0 {
                        coord.x = self.container[coord.y - 1].len() - 1;
                        coord.y -= 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        self.put_on(coord.x as usize, coord.y as usize, elem);
                    }
                }
                'e' => {
                    if coord.y < self.max_y - 1 {
                        coord.x = 0;
                        coord.y += 1;
                        self.lines_count += 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        self.put_on(coord.x as usize, coord.y as usize, elem);
                    }
                }
                _ => {}
            }
        }

        pub fn destroy_pointer(&mut self, coord: &mut Coord, direction: char) {
            if direction == 'l' {
                self.put_on(coord.x, coord.y, ' ');
            } else if direction == 'e' {
                self.put_on(coord.x, coord.y, '\n');
            } else if direction == 'd' {
                if coord.x >= self.container[coord.y].len() {
                    return;
                }
                self.put_on(coord.x, coord.y, ' ');
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
                                        }
                                        break;
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
                            match code {
                                KeyCode::Esc => {
                                    command_buffer.clear();
                                    command_mode = true;
                                }
                                KeyCode::Up => {
                                    self.destroy_pointer(&mut coord, 'd');
                                    self.moove_on(&mut coord, 'u', '|');
                                }
                                KeyCode::Down => {
                                    self.destroy_pointer(&mut coord, 'd');
                                    self.moove_on(&mut coord, 'd', '|');
                                }
                                KeyCode::Left => {
                                    self.destroy_pointer(&mut coord, 'l');
                                    self.moove_on(&mut coord, 'l', '|');
                                }
                                KeyCode::Right => {
                                    self.destroy_pointer(&mut coord, 'd');
                                    self.moove_on(&mut coord, 'r', '|');
                                }
                                KeyCode::Char(c) => {
                                    self.put_on(self.pointer_pos.x, self.pointer_pos.y, c);
                                    self.write_on(&mut coord, 'r', '|');
                                }
                                KeyCode::Backspace => {
                                    self.destroy_pointer(&mut coord, 'l');
                                    self.write_on(&mut coord, 'l', '|');
                                }
                                KeyCode::Enter => {
                                    self.destroy_pointer(&mut coord, 'e');
                                    self.write_on(&mut coord, 'e', '|');
                                }
                                _ => {}
                            }
                            self.super_display();
                        }
                        //execute!(io::stdout(), terminal::Clear(ClearType::All))?;
                    }
                }
            }
            terminal::disable_raw_mode()?;
            Ok(())
        }
        pub fn display_command(&self, command: &String) {
            self.super_display();
            execute!(io::stdout(), MoveTo(0, self.max_y as u16)).unwrap();
            print!("{}", command);
            io::stdout().flush().unwrap();
        }
        pub fn super_display(&self) {
            execute!(
                io::stdout(),
                MoveTo((self.max_x - 10) as u16, self.max_y as u16)
            )
            .unwrap();
            self.pointer_pos.display();
            for (y, row) in self.container.iter().enumerate() {
                execute!(io::stdout(), MoveTo(0, y as u16)).unwrap();
                for (x, cell) in row.iter().enumerate() {
                    if self.pointer_pos.x == x && self.pointer_pos.y == y {
                        print!("{}", cell.with(Color::Red));
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
}
