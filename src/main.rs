use tetrust::engine::engine;
use tetrust::view;
use tetrust::playfield;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    match termion::terminal_size() {
        Ok(v) => {
            let (width, height) = v;

            if width < 80 || height < 25 {
                println!("Terminal size should be greater than (W:80, H:25). Given (W:{}, H:{})", width, height);
                return;
            }
        }
        Err(e) => {
            println!("Cannot read terminal dimensions: {:?}", e);
            return
        },
    }

    let playfield = playfield::Playfield::new(Default::default());
    let view = view::ConsoleView{};
    let mut game = engine::new_game(playfield, &view);

    let (timer_tx, rx) = mpsc::channel();
    let keyboard_tx = timer_tx.clone();

    thread::spawn(move || {
        /* timeout generator */
        const FRAME_INTERVAL: u64 = 1000000 / 60;

        loop {
            let _ = timer_tx.send(engine::Event::Timeout);
            thread::sleep(Duration::from_micros(FRAME_INTERVAL));
        }
    });

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();

    thread::spawn(move || {
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Left => {let _ = keyboard_tx.send(engine::Event::KeyLeft);},
                Key::Right => {let _ = keyboard_tx.send(engine::Event::KeyRight);},
                Key::Down => {let _ = keyboard_tx.send(engine::Event::KeyDown);},
                Key::Up => {let _ = keyboard_tx.send(engine::Event::KeyTurn);},
                Key::Char(' ') => {let _ = keyboard_tx.send(engine::Event::KeyDrop);},
                Key::Char('q') | Key::Ctrl('z') | Key::Ctrl('c') => {let _ = keyboard_tx.send(engine::Event::KeyExit);},
                _ => { /* do nothing */ }
            }
        }
    });

    while engine::get_state(&game) != engine::State::End {
        let event = rx.recv().unwrap();
        engine::calculate_frame(&mut game, event);
        engine::draw_frame(&mut game);

        if event == engine::Event::KeyExit {
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
