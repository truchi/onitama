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
    // dbg!(C.apply(Right(0)));
    // dbg!(Three.apply(Up(1)));
    // dbg!(Square(C, Three).apply(Move(Up(2), Right(0))));
    enter();
    game_loop();
    leave();
}

pub fn game_loop() {
    let time = Instant::now();
    let spf = Duration::from_millis(500);
    let (mut width, mut height) = size();

    loop {
        x::execute!(stdout(), x::Clear(x::ClearType::All), x::MoveTo(0, 0)).unwrap();
        // printcr!("hello {:?}", Instant::now() - time);

        while let Some(event) = poll() {
            match event {
                x::Event::Key(event) => {
                    // printcr!("{:?}", event);
                    if event.code == x::KeyCode::Esc {
                        leave();
                        std::process::exit(0);
                    }
                }
                x::Event::Mouse(event) => {}
                x::Event::Resize(w, h) => {
                    width = w;
                    height = h;
                }
            }
        }
        GameUI::new(width, height).render();
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
