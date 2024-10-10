use super::buffer::{Buffer, Coord};

use std::io;

use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

impl Buffer {
    pub fn put_on(&mut self, x: usize, y: usize, elem: char) {
        if elem == '£' {
            self.container.push(vec![' ']);
            execute!(io::stdout(), Clear(ClearType::All)).unwrap();
        } else if x < self.max_x as usize && y < self.max_y as usize {
            if y > self.container.len() {
                while y >= self.container.len() {
                    self.container.push(vec![' ']);
                }
            }
            if x >= self.container[y].len() {
                self.container[y].push(elem);
            }
            if x == self.container[y].len() && x > 0 {
                self.container[y][x - 1] = elem;
                self.pointer_pos.x -= 1;
            } else {
                self.container[y][x] = elem;
            }
        }
    }
    pub fn insert_on(&mut self, coord: &mut Coord, elem: char) {
        self.container[coord.y].insert(coord.x, elem);
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
                    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
                }
            }
            'd' => {
                if coord.y < self.lines_count {
                    coord.y += 1;
                    coord.x = self.container[coord.y].len() - 1;
                    self.pointer_pos.x = coord.x;
                    self.pointer_pos.y = coord.y;
                    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
                }
            }
            'r' => {
                if coord.x < self.container[coord.y].len() - 1 {
                    coord.x += 1;
                    if elem == '|' && coord.x >= self.container[coord.y].len() {
                        return;
                    }
                    self.pointer_pos.x = coord.x;
                    self.pointer_pos.y = coord.y;
                    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
                } else if coord.x == self.max_x - 1 && coord.y < self.max_y - 1 {
                    coord.x = 0;
                    coord.y += 1;
                    self.pointer_pos.x = coord.x;
                    self.pointer_pos.y = coord.y;
                    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
                }
            }
            'l' => {
                if coord.x > 0 {
                    coord.x -= 1;
                    self.pointer_pos.x = coord.x;
                    self.pointer_pos.y = coord.y;
                    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
                } else if coord.x == 0 && coord.y > 0 {
                    coord.x = self.container[coord.y].len() - 1;
                    coord.y -= 1;
                    self.pointer_pos.x = coord.x;
                    self.pointer_pos.y = coord.y;
                    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
                }
            }
            'e' => {
                if coord.y < self.max_y {
                    coord.x = 0;
                    coord.y += 1;
                    self.pointer_pos.x = coord.x;
                    self.pointer_pos.y = coord.y;
                    self.lines_count += 1;
                    self.put_on((coord.x) as usize, coord.y as usize, '£');
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
                    if coord.x < self.container[coord.y].len() - 1 && coord.x > 0 {
                        self.insert_on(coord, elem);
                    } else {
                        self.put_on((coord.x) as usize, coord.y as usize, elem);
                    }
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
            if coord.x > 0 {
                self.container[coord.y].remove(coord.x);
                self.pointer_pos.x -= 1;
                execute!(io::stdout(), Clear(ClearType::All)).unwrap();
            }
        } else if direction == 'e' {
            self.put_on(coord.x, coord.y, ' ');
        } else if direction == 'd' {
            if coord.x >= self.container[coord.y].len() {
                return;
            }
            self.put_on(coord.x, coord.y, ' ');
        }
    }
}
