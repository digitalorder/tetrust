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
    let playfield = playfield::Playfield::new(Default::default());
    let view = view::ConsoleView{};
    let mut game = engine::new_game(playfield, &view);

    let (timer_tx, rx) = mpsc::channel();
    let keyboard_tx = timer_tx.clone();

    thread::spawn(move || {
        /* timeout generator */
        let interval = 2000;

        loop {
            let _ = timer_tx.send(engine::Event::Timeout);
            thread::sleep(Duration::from_millis(interval));
        }
    });

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    thread::spawn(move || {
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Left => {let _ = keyboard_tx.send(engine::Event::KeyLeft);},
                Key::Right => {let _ = keyboard_tx.send(engine::Event::KeyRight);},
                Key::Down => {let _ = keyboard_tx.send(engine::Event::KeyDown);},
                Key::Char(' ') => {let _ = keyboard_tx.send(engine::Event::KeyTurn);},
                Key::Char('q') | Key::Ctrl('z') | Key::Ctrl('c') => {let _ = keyboard_tx.send(engine::Event::KeyExit);},
                _ => { /* do nothing */ }
            }
        }
    });

    while engine::get_state(&game) != engine::State::End {
        let event = rx.recv().unwrap();
        engine::calculate_frame(&mut game, event);
        engine::draw_frame(&game);

        if event == engine::Event::KeyExit {
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
