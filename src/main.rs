extern crate cursive;
use cursive::{Cursive, Printer};
use cursive::theme::{ColorStyle, Color};
use cursive::views::{Dialog, LinearLayout};
use cursive::views::Canvas;
use cursive::view::Boxable;
use cursive::traits::*;
use cursive::event::{Event, EventResult, Key};
use std::net::{TcpListener, TcpStream};
use std::env;
use std::env::Args;
use std::thread;
use std::str::FromStr;
use std::io::Write;
use std::process::exit;

mod game_obj;
mod message_system;

pub struct Game {
    state: game_obj::Game_State_Id,
    my_grid: game_obj::Grid,
    target_grid: game_obj::Grid,
    history: Vec<String>,
    msg: String,
    stream: Option<TcpStream>,
    listener: Option<TcpListener>,
    ready_check: (bool, bool),
    msg_sys: Option<message_system::MessageSystem>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: game_obj::Game_State_Id::Edit,
            my_grid: game_obj::Grid::new(),
            target_grid: game_obj::Grid::new(),
            history: Vec::new(),
            msg: "Insert Coin".to_string(),
            stream: None,
            listener: None,
            ready_check: (false, false),
            msg_sys: None,
        }
    }

    pub fn start<'a>(&'a mut self, mut arg: Args) {
        let mut addr: String = "".to_string();
        if arg.len() > 1 {
            addr = arg.nth(1).unwrap().to_string();
            self.stream = match TcpStream::connect(addr) {
                Ok(_stream) => {
                    self.msg = "CONNECTED!".to_string();
                    self.msg_sys = Some(message_system::MessageSystem::new(_stream.take().unwrap()));
                    Some(_stream)
                },
                Err(e) => {
                    println!("Err: {}", e);
                    self.msg = e.to_string();
                    None
                }
            }
        }
        else {
            // I must Host!
            addr = "localhost:52525".to_string();
            println!("Waiting for connection...");
            self.listener = Some(TcpListener::bind(addr).unwrap());
            self.stream = match self.listener.take().unwrap().accept() {
                Ok((_stream, _addr)) => {
                    self.msg = _addr.to_string();
                    self.msg_sys = Some(message_system::MessageSystem::new(_stream.take().unwrap())
                        .with_task(|stream| {

                        }));
                    Some(_stream)
                },
                Err(e) => {
                    println!("Err: {}", e);
                    self.msg = e.to_string();
                    None
                },
            };
        }


        self.target_grid.color_bg = Color::Rgb(0, 0, 0);
    }


    pub fn step(&mut self) {
        /*let l = self.listener.unwrap();
        for stream in l.incoming() {
            let stream = stream.unwrap();
            thread::spawn(move || {
                //handle_client(stream).unwrap();
            });
        }*/
        match self.state {
            game_obj::Game_State_Id::Edit => self.edit_state_loop(),
            game_obj::Game_State_Id::MyTurn => self.myturn_state_loop(),
            game_obj::Game_State_Id::TheirTurn => self.theirturn_state_loop(),
            game_obj::Game_State_Id::GameOver => self.gameover_state_loop(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> EventResult {
        match self.state {
            game_obj::Game_State_Id::Edit => self.edit_state_handle_event(event),
            game_obj::Game_State_Id::MyTurn => self.myturn_state_handle_event(event),
            game_obj::Game_State_Id::TheirTurn => self.theirturn_state_handle_event(event),
            game_obj::Game_State_Id::GameOver => self.gameover_state_handle_event(event),
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Edit State Logic
    ////////////////////////////////////////////////////////////////////////////////////////////////

    fn edit_state_loop(&mut self) {
        if self.ready_check == (true, true) {
            self.state = game_obj::Game_State_Id::MyTurn;
        }
    }

    fn edit_state_handle_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Enter) => {
                if self.my_grid.ships.len() > 1 {
                    for s in 0..self.my_grid.ships.len() {
                        for p in 0..self.my_grid.ships.last().unwrap().positions.len() {
                            if self.my_grid.ships[s].bounds_check_fail(self.my_grid.ships.last().unwrap().positions[p].x, self.my_grid.ships.last().unwrap().positions[p].y) {
                                return EventResult::Consumed(None)
                            }
                        }
                    }
                }
                match self.my_grid.ships.len() {
                    0 => { // Destroyer
                        self.my_grid.ships.push(game_obj::Ship::new(
                            2,
                            vec![
                                game_obj::Hit::new(0, 0, false),
                                game_obj::Hit::new(0, 1, false)
                            ],
                            false,
                        ));
                        self.msg = "Destroyer".to_string();
                    },
                    1 => { // Submarine
                        self.my_grid.ships.push(game_obj::Ship::new(
                            3,
                            vec![
                                game_obj::Hit::new(0, 0, false),
                                game_obj::Hit::new(0, 1, false),
                                game_obj::Hit::new(0, 2, false)
                            ],
                            false,
                        ));
                        self.msg = "Submarine".to_string();
                    },
                    2 => { // Cruiser
                        self.my_grid.ships.push(game_obj::Ship::new(
                            3,
                            vec![
                                game_obj::Hit::new(0, 0, false),
                                game_obj::Hit::new(0, 1, false),
                                game_obj::Hit::new(0, 2, false)
                            ],
                            false,
                        ));
                        self.msg = "Cruiser".to_string();
                    },
                    3 => { // Battleship
                        self.my_grid.ships.push(game_obj::Ship::new(
                            4,
                            vec![
                                game_obj::Hit::new(0, 0, false),
                                game_obj::Hit::new(0, 1, false),
                                game_obj::Hit::new(0, 2, false),
                                game_obj::Hit::new(0, 3, false)
                            ],
                            false,
                        ));
                        self.msg = "Battleship".to_string();
                    },
                    4 => { // Carrier
                        self.my_grid.ships.push(game_obj::Ship::new(
                            5,
                            vec![
                                game_obj::Hit::new(0, 0, false),
                                game_obj::Hit::new(0, 1, false),
                                game_obj::Hit::new(0, 2, false),
                                game_obj::Hit::new(0, 3, false),
                                game_obj::Hit::new(0, 4, false)
                            ],
                            false,
                        ));
                        self.msg = "Carrier".to_string();
                    },
                    _ => {
                        self.stream.take().unwrap().write("Ready".as_bytes());

                        self.msg = "READY!".to_string();
                    },
                }

            },
            Event::Key(Key::Up) => {
                if self.my_grid.ships.last().unwrap().positions.first().unwrap().y - 1 >=  0 {
                    for p in 0..self.my_grid.ships.last_mut().unwrap().positions.len() {
                        self.my_grid.ships.last_mut().unwrap().get_position_mut(p).unwrap().y_mod(-1);
                    }
                }
            },
            Event::Key(Key::Down) => {
                if self.my_grid.ships.last().unwrap().positions.last().unwrap().y + 1 < 10 {
                    for p in 0..self.my_grid.ships.last_mut().unwrap().positions.len() {
                        self.my_grid.ships.last_mut().unwrap().get_position_mut(p).unwrap().y_mod(1);
                    }
                }
            },
            Event::Key(Key::Left) => {
                if self.my_grid.ships.last().unwrap().positions.first().unwrap().x - 1 >=  0 {
                    for p in 0..self.my_grid.ships.last_mut().unwrap().positions.len() {
                        self.my_grid.ships.last_mut().unwrap().get_position_mut(p).unwrap().x_mod(-1);
                    }
                }
            },
            Event::Key(Key::Right) => {
                if self.my_grid.ships.last().unwrap().positions.last().unwrap().x + 1 < 10 {
                    for p in 0..self.my_grid.ships.last_mut().unwrap().positions.len() {
                        self.my_grid.ships.last_mut().unwrap().get_position_mut(p).unwrap().x_mod(1);
                    }
                }
            },
            Event::Key(Key::Tab) => {
                self.my_grid.ships.last_mut().unwrap().transpose(0, 0, 10, 10);
            }
            _ => (),
        }

        EventResult::Consumed(None)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // MyTurn State Logic
    ////////////////////////////////////////////////////////////////////////////////////////////////

    fn myturn_state_loop(&mut self) {

    }

    fn myturn_state_handle_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Tab) => {
                self.state = game_obj::Game_State_Id::MyTurn;
            },
            _ => (),
        }
        EventResult::Consumed(None)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // TheirTurn State Logic
    ////////////////////////////////////////////////////////////////////////////////////////////////

    fn theirturn_state_loop(&mut self) {

    }

    fn theirturn_state_handle_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Tab) => {
                self.state = game_obj::Game_State_Id::GameOver;
            },
            _ => (),
        }
        EventResult::Consumed(None)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // GameOver State Logic
    ////////////////////////////////////////////////////////////////////////////////////////////////

    fn gameover_state_loop(&mut self) {

    }

    fn gameover_state_handle_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Tab) => {
                self.state = game_obj::Game_State_Id::Edit;
            },
            _ => (),
        }
        EventResult::Consumed(None)
    }
}

impl View for Game {
    fn draw(&self, printer: &Printer) {
        match self.state {
            game_obj::Game_State_Id::Edit => self.my_grid.draw(printer),
            game_obj::Game_State_Id::MyTurn => self.target_grid.draw(printer),
            game_obj::Game_State_Id::TheirTurn => self.my_grid.draw(printer),
            game_obj::Game_State_Id::GameOver => printer.print((0,0), "YOU DIED"),
        }

        printer.print((0,10), &self.msg);

    }

    fn on_event(&mut self, event: Event) -> EventResult {
        let line = format!("{:?}", event);
        self.history.push(line);
        //println!("Key: {:?}", event);
        while self.history.len() > 10 {
            self.history.remove(0);
        }

        self.handle_event(event)
    }
}

fn main() {
    let mut game = Game::new();
    game.start(env::args());

    let mut siv = Cursive::new();

    //siv.add_layer(Canvas::new(()).with_draw(game.draw).with_on_event(game.on_event).fixed_size((10, 11)));
    siv.add_layer(Dialog::around(LinearLayout::vertical().child(
        game
            .fixed_size((10, 11)))
    ).title("Battleship!").with_id("game"));
    //siv.add_layer(game.fixed_size((10, 11)).with_id("game"));

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
    /*while siv.is_running() {
        /*match siv.find_id::<Canvas>("game") {
            Some(mut g) => {
                //g.step();
                siv.step();
            },
            None => {
                println!("The table has been flipped; the game is gone.");
                siv.quit();
            }
        };*/
        game.step();
        siv.step();

    }*/
}