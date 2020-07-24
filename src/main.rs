use tetrust::engine::engine;
use tetrust::view;
use tetrust::playfield;
use tetrust::fall::FRAME_RATE;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use clap::{App, value_t};

fn do_game(no_ghost: bool, level: u8, next_queue_size: u8, mode: engine::Mode) {
    let playfield = playfield::Playfield::new(Default::default());
    let config = engine::Config{no_ghost: no_ghost, level: level, next_queue_size: next_queue_size, mode: mode};
    let mut game = engine::new_game(config, playfield);

    let (timer_tx, rx) = mpsc::channel();
    let keyboard_tx = timer_tx.clone();

    thread::spawn(move || {
        /* timeout generator */
        const FRAME_INTERVAL: u64 = 1000000 / FRAME_RATE as u64;

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
                Key::Char('h') => {let _ = keyboard_tx.send(engine::Event::KeyHold);},
                Key::Char('q') | Key::Ctrl('z') | Key::Ctrl('c') => {let _ = keyboard_tx.send(engine::Event::KeyExit);},
                _ => { /* do nothing */ }
            }
        }
    });

    while !engine::is_finished(&game) {
        let mut view = view::ConsoleView{};
        let event = rx.recv().unwrap();
        engine::calculate_frame(&mut game, event.clone());
        engine::draw_frame(&mut game, &mut view);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

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

    let matches = App::new("tetrust")
                    .version(env!("CARGO_PKG_VERSION"))
                    .author("Denis Vasilkovskii <digitalorder>")
                    .about("Classic tetris game implemented in Rust")
                    .args_from_usage(
                        "-g, --no-ghost 'Disables ghost tetro for easy dropping'
                         -l, --level [level] 'Start level (0-29)'
                         -n, --next-queue-size [size] 'Upcoming tetriminos queue size (0-4)'
                         -m, --mode [marathon,sprint] 'Game mode'")
                    .get_matches();

    let no_ghost = matches.is_present("no-ghost");
    let level = value_t!(matches, "level", u8).unwrap_or(0);
    let next_queue_size = value_t!(matches, "next-queue-size", u8).unwrap_or(4);
    let mode_str = value_t!(matches, "mode", String).unwrap_or("marathon".to_string());
    let mode = if mode_str == "marathon" {
        engine::Mode::Marathon
    } else if mode_str == "sprint" {
        engine::Mode::Sprint
    } else {
        println!("Unknown game mode {}. Use -h to list supported modes.", mode_str);
        std::process::exit(-1);
    };
    println!("no ghost tetro: {} level: {} queue size: {} mode: {}", no_ghost, level, next_queue_size, mode);
    do_game(no_ghost, level, next_queue_size, mode);
}
