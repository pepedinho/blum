pub mod buffer_mod {
    use lazy_static::lazy_static;
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
        pub container: Vec<Vec<char>>,
    }

    impl Coord {
        pub fn init() -> Coord {
            let x = 0;
            let y = 0;
            Coord { x, y }
        }
        pub fn display(&self) {
            println!("x: {} y: {}", self.x, self.y);
        }
    }

    impl Buffer {
        pub fn init(x: usize, y: usize) -> Buffer {
            let pointer_pos = Coord::init();
            let container: Vec<Vec<char>> = vec![vec![' '; x as usize]; y as usize];
            let max_x = x;
            let max_y = y;
            Buffer {
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
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        if elem == '|' {
                            self.update_prev_char(coord, 'u');
                        }
                        self.put_on(coord.x as usize, (coord.y) as usize, elem);
                    }
                }
                'd' => {
                    if coord.y < self.max_y - 1 {
                        coord.y += 1;
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        if elem == '|' {
                            self.update_prev_char(coord, 'd');
                        }
                        self.put_on(coord.x as usize, (coord.y) as usize, elem);
                    }
                }
                'r' => {
                    if coord.x < self.max_x - 1 {
                        coord.x += 1;
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
                    if coord.x > 0 {
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
                        coord.x = self.max_x - 1;
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
                        self.pointer_pos.x = coord.x;
                        self.pointer_pos.y = coord.y;
                        self.put_on((coord.x) as usize, coord.y as usize, elem);
                    }
                }
                _ => {}
            }
        }

        pub fn destroy_pointer(&mut self, coord: &mut Coord, direction: char) {
            if direction == 'l' {
                self.put_on(coord.x, coord.y, ' ');
            } else if direction == 'e' {
                self.put_on(coord.x, coord.y, ' ');
            }
        }
        pub fn listen(&mut self) -> io::Result<()> {
            let mut coord = Coord::init();
            execute!(io::stdout(), terminal::Clear(ClearType::All));
            terminal::enable_raw_mode()?;
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
                            KeyCode::Esc => break,
                            KeyCode::Up => {
                                self.destroy_pointer(&mut coord, 'l');
                                self.moove_on(&mut coord, 'u', '|');
                            }
                            KeyCode::Down => {
                                self.destroy_pointer(&mut coord, 'l');
                                self.moove_on(&mut coord, 'd', '|');
                            }
                            KeyCode::Left => {
                                self.destroy_pointer(&mut coord, 'l');
                                self.moove_on(&mut coord, 'l', '|');
                            }
                            KeyCode::Right => {
                                self.destroy_pointer(&mut coord, 'l');
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
                        //execute!(io::stdout(), terminal::Clear(ClearType::All))?;
                        self.super_display();
                    }
                }
            }
            Ok(())
        }
        pub fn super_display(&self) {
            self.pointer_pos.display();
            for (y, row) in self.container.iter().enumerate() {
                execute!(io::stdout(), MoveTo(0, y as u16)).unwrap();
                for (x, cell) in row.iter().enumerate() {
                    if self.pointer_pos.x == x && self.pointer_pos.y == y {
                        print!("{}", cell.with(Color::Red));
                    } else {
                        print!("{}", cell);
                    }
                }
                println!();
            }
            println!();
        }
    }
}
