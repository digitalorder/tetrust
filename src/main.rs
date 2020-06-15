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

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();

    thread::spawn(move || {
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Left => {let _ = keyboard_tx.send(engine::Event::KeyLeft);},
                Key::Right => {let _ = keyboard_tx.send(engine::Event::KeyRight);},
                _ => {let _ = keyboard_tx.send(engine::Event::KeyExit);},
            }
        }
    });

    while engine::get_state(&game) != engine::State::End {
        let event = rx.recv().unwrap();
        engine::calculate_frame(&mut game, event);
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
        println!("Event: {}\r", event);
        engine::draw_frame(&game);

        if event == engine::Event::KeyExit {
            break;
        }
    }
}
