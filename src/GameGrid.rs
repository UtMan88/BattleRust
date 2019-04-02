extern crate cursive;
use cursive::Printer;
use cursive::theme::{ColorStyle, Color};

pub struct Hit {
    pub x: u8,
    pub y: u8,
    pub is_hit: bool,
}

impl Hit {
    pub fn new( _x:u8, _y:u8, _is_hit:bool) -> Hit {
        Hit{
            x: _x,
            y: _y,
            is_hit: _is_hit
        }
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
}

pub struct Grid {
    pub hits:  Vec<Hit>,
    pub ships: Vec<Ship>
}

impl Grid {
    pub fn new() -> Grid {
        Grid{
            hits: Vec::new(),
            ships: Vec::new()
        }
    }

    pub fn is_hit<'a>(&'a self, x: u8, y: u8) -> Option<bool> {
        let _hits = &self.hits;
        for h in _hits {
            if h.x == x && h.y == y {
                return Some(h.is_hit);
            }
        }
        None
    }

    fn back_color<'a>(&'a self, x: u8, y: u8) -> Color {
        let _ships = &self.ships;
        for s in _ships {
            for p in &s.positions {
                if p.x == x && p.y ==y {
                    return Color::Rgb(128, 128, 128); // Gray
                }
            }
        }
        Color::Rgb(127, 255, 212) // Aquamarine
    }

    pub fn draw<'a>(&'a self, p: &Printer) {
        let x_size = 10;
        let y_size = 10;

        for x in 0..x_size {
            for y in 0..y_size {
                let h = self.is_hit(x, y);
                let style = ColorStyle::Custom {
                    front: match h {
                        None => Color::Rgb(224, 255, 255),
                        Some(_h) => {
                            if h.unwrap() == true {
                                Color::Rgb(255, 0, 0)
                            } else {
                                Color::Rgb(255, 255, 255)
                            }
                        }
                    },
                    back: self.back_color(x, y),
                };
                p.with_color(style, |printer| { printer.print((x, y), if h != None {
                    "X"
                } else {
                    " "
                }
                ); });
            }
        }
    }
}