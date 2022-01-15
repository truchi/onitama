mod ui;

use super::*;
use ui::*;

mod x {
    pub use crossterm::cursor::Hide;
    pub use crossterm::cursor::MoveTo;
    pub use crossterm::cursor::Show;
    pub use crossterm::event::poll;
    pub use crossterm::event::read;
    pub use crossterm::event::DisableMouseCapture;
    pub use crossterm::event::EnableMouseCapture;
    pub use crossterm::event::Event;
    pub use crossterm::event::KeyCode;
    pub use crossterm::event::MouseButton;
    pub use crossterm::event::MouseEventKind;
    pub use crossterm::execute;
    pub use crossterm::queue;
    pub use crossterm::style::Color;
    pub use crossterm::style::Color::*;
    pub use crossterm::style::Stylize;
    pub use crossterm::terminal::disable_raw_mode;
    pub use crossterm::terminal::enable_raw_mode;
    pub use crossterm::terminal::size;
    pub use crossterm::terminal::Clear;
    pub use crossterm::terminal::ClearType;
    pub use crossterm::terminal::EnterAlternateScreen;
    pub use crossterm::terminal::LeaveAlternateScreen;
}

use std::io::stdout;
use std::io::Write;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

macro_rules! printcr {
    () => (print!("\r\n"));
    ($($arg:tt)*) => { {
        print!($($arg)*);
        print!("\r\n")
    }};
}

pub fn main() {
    enter();
    game_loop();
    leave();
}

pub fn game_loop() {
    let time = Instant::now();
    let spf = Duration::from_millis(100);
    let (mut width, mut height) = size();

    let mut game = Game::new([8, 9], [10, 11], 12);
    let mut ui = GameUI::new(width, height, game);
    ui.render();

    loop {
        while let Some(event) = poll() {
            match event {
                x::Event::Key(event) =>
                    if event.code == x::KeyCode::Esc {
                        leave();
                        exit(0);
                    },
                x::Event::Mouse(event) => {
                    if event.kind == x::MouseEventKind::Down(x::MouseButton::Left) {
                        if let Some(play) = ui
                            .handle_click((event.column, event.row), |card, src, dest| {
                                Play::Card { card, src, dest }
                            })
                        {
                            match game.play(play) {
                                State::Won(winner) => {
                                    leave();
                                    println!("{:?} wins", winner);
                                    exit(0);
                                }
                                State::Draw => {
                                    leave();
                                    println!("Draw");
                                    exit(0);
                                }
                                _ => {
                                    ui = GameUI::new(width, height, game);
                                    ui.render();
                                }
                            }
                        }
                    }
                }
                x::Event::Resize(width, height) => {
                    ui.set_size(width, height);
                    ui.render();
                }
            }
        }
        sleep(spf);
        stdout().flush().unwrap();
    }
}

fn poll() -> Option<x::Event> {
    if x::poll(Duration::from_millis(0)).unwrap() {
        Some(x::read().unwrap())
    } else {
        None
    }
}

fn size() -> (u16, u16) {
    x::size().unwrap()
}

fn enter() {
    x::execute!(
        stdout(),
        x::EnterAlternateScreen,
        x::EnableMouseCapture,
        x::Hide
    )
    .unwrap();
    x::enable_raw_mode().unwrap();
}

fn leave() {
    x::disable_raw_mode().unwrap();
    x::execute!(
        stdout(),
        x::LeaveAlternateScreen,
        x::DisableMouseCapture,
        x::Show
    )
    .unwrap();
}
