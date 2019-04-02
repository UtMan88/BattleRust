extern crate cursive;
use cursive::Printer;
use cursive::theme::{ColorStyle, Color};

pub enum Game_State_Id {
    Edit,
    MyTurn,
    TheirTurn,
    GameOver
}

pub struct Hit {
    pub x: i32,
    pub y: i32,
    pub is_hit: bool,
}

impl Hit {
    pub fn new( _x:i32, _y:i32, _is_hit:bool) -> Hit {
        Hit{
            x: _x,
            y: _y,
            is_hit: _is_hit
        }
    }

    pub fn x_mod(&mut self, m: i32) {
        self.x += m;
    }

    pub fn y_mod(&mut self, m: i32) {
        self.y += m;
    }

    pub fn set_is_hit(&mut self, value: bool) {
        self.is_hit = value;
    }
}

pub struct Ship {
    pub len: usize,
    pub positions: Vec<Hit>,
    pub is_sunk: bool,
}

impl Ship {
    pub fn new( _len:usize, _positions:Vec<Hit>, _is_sunk:bool) -> Ship {
        Ship{
            len: _len,
            positions: _positions,
            is_sunk:_is_sunk
        }
    }

    pub fn get_position_mut(&mut self, index: usize) -> Option<&mut Hit> {
        self.positions.get_mut(index)
    }

    pub fn transpose(&mut self, grid_min_x: i32, grid_min_y: i32, grid_max_x: i32, grid_max_y: i32) {
        let fx = self.positions.first().unwrap().x;
        let fy = self.positions.first().unwrap().y;
        let lx = self.positions.last().unwrap().x;
        let ly = self.positions.last().unwrap().y;
        if lx - fx == 0 { // Vertical to Horizontal
            if lx + self.len as i32 > grid_max_x {
                return;
            }
            for p in 0..self.len {

                self.get_position_mut(p).unwrap().x_mod(p as i32);
                self.get_position_mut(p).unwrap().y = self.positions.first().unwrap().y;
            }

        } else if ly - fy == 0 { // Horizontal to Vertical
            if ly + self.len as i32 > grid_max_y {
                return;
            }
            for p in 0..self.len {
                self.get_position_mut(p).unwrap().x = self.positions.first().unwrap().x;
                self.get_position_mut(p).unwrap().y_mod(p as i32);
            }
        }
        else { println!("Couldn't Transpose!") }
    }

    pub fn bounds_check_fail(&self, x: i32, y: i32) -> bool {
        for p in self.positions.iter() {
            if p.x == x && p.y == y {
                return true;
            }
        }

        false
    }
}

pub struct Grid {
    pub cursor: Hit,
    pub hits:  Vec<Hit>,
    pub ships: Vec<Ship>,
    pub color_bg: Color,
    pub color_ship: Color,
    pub color_hit: Color,
    pub color_miss: Color,
    pub color_cursor: Color,
}

impl Grid {
    pub fn new() -> Grid {
        Grid{
            cursor: Hit::new(0, 0, false),
            hits: Vec::new(),
            ships: Vec::new(),
            color_bg: Color::Rgb(127, 255, 212),
            color_ship: Color::Rgb(128, 128, 128),
            color_cursor: Color::Rgb(0, 255, 0),
            color_hit: Color::Rgb(255, 0, 0),
            color_miss: Color::Rgb(255, 255, 255),
        }
    }

    pub fn ships_mut(&mut self) -> &Vec<Ship> {
        &self.ships
    }

    pub fn is_hit<'a>(&'a self, x: i32, y: i32) -> Option<bool> {
        let _hits = &self.hits;
        for h in _hits {
            if h.x == x && h.y == y {
                return Some(h.is_hit);
            }
        }
        None
    }

    fn back_color<'a>(&'a self, x: i32, y: i32) -> Color {
        if self.cursor.is_hit == true && self.cursor.x == x && self.cursor.y == y {
            return self.color_cursor;
        }
        let _ships = &self.ships;
        for s in _ships {
            for p in &s.positions {
                if p.x == x && p.y ==y {
                    return self.color_ship;
                }
            }
        }
        self.color_bg
    }

    pub fn draw<'a>(&'a self, p: &Printer) {
        let x_size = 10;
        let y_size = 10;

        for x in 0..x_size {
            for y in 0..y_size {
                let h = self.is_hit(x, y);
                let style = ColorStyle::Custom {
                    front: match h {
                        None => self.color_bg,
                        Some(_h) => {
                            if h.unwrap() == true {
                                self.color_hit
                            } else {
                                self.color_miss
                            }
                        }
                    },
                    back: self.back_color(x, y),
                };
                p.with_color(style, |printer| { printer.print((x, y),
                                                              if h != None {
                                                                  "X"
                                                              } else {
                                                                  " "
                                                              }
                ); });
            }
        }
    }
}