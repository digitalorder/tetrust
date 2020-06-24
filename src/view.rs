use crate::playfield::{Playfield, FieldTetromino, WIDTH, HEIGHT, Coords};
use crate::figures::figures::{Shape, Tetromino, LAYOUT_HEIGHT, LAYOUT_WIDTH};
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
extern crate termion;

pub type Row = [char; WIDTH as usize];
pub struct ConsoleView {
}

pub trait View {
    fn show_row(&self, playfield: &Playfield, row: i8, active_tetro: &FieldTetromino) -> Row;
    fn show_playfield(&self, playfield: &Playfield, active_tetro: &FieldTetromino);
    fn show_static(&self, level: u8, score: u32, lines: u32);
    fn show_next(self: &Self, tetro: &Tetromino);
    /* TODO: add rows iterator */
}

impl View for ConsoleView {
    fn show_row(&self, playfield: &Playfield, row: i8, active_tetro: &FieldTetromino) -> Row {
        let mut result: Row = [' '; WIDTH as usize];
        for i in 0..WIDTH as usize {
            let (shape, is_active) = playfield.shape_at(&Coords{row: row, col: i as i8}, active_tetro);
            let mut value: char = match shape {
                Shape::NoShape => ' ',
                Shape::OShape => 'o',
                Shape::IShape => 'i',
                Shape::TShape => 't',
                Shape::JShape => 'j',
                Shape::LShape => 'l',
                Shape::SShape => 's',
                Shape::ZShape => 'z',
            };

            if is_active {
                value.make_ascii_uppercase();
            }
            result[i] = value;
        }
        result
    }

    fn show_playfield(&self, playfield: &Playfield, active_tetro: &FieldTetromino) {
        for row in 0..HEIGHT {
            print!("{}", termion::cursor::Goto(2, 3 + (row as u16)));
            for col in 0..WIDTH {
                let (shape, is_active) = playfield.shape_at(&Coords{row: HEIGHT - row - 1, col: col as i8}, active_tetro);
                let color = convert_to_color(shape, is_active);
                print!("{}  {}", termion::color::Bg(color), termion::color::Bg(termion::color::Black));
            }
        }
        print!("{}", termion::color::Bg(termion::color::Black));
        print!("{}", termion::cursor::Goto(1, HEIGHT as u16 + 4));
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
    }

    fn show_static(self: &Self, level: u8, score: u32, lines: u32) {
        print!("{}TETRUST v{}. Move: ⬅️ ⬇️ ➡️  Rotate: ⬆️  Drop: Spacebar. Exit: q\n\r",
               termion::cursor::Goto(1, 1), env!("CARGO_PKG_VERSION"));
        print!("┌────────────────────┐\n\r");
        for _ in 0..HEIGHT {
            print!("│                    │\n\r");
        }
        print!("└────────────────────┘\n\r");
        print!("{}Level: {} Score: {} Lines: {}", termion::cursor::Goto(26, 2), level, score, lines);
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
    }

    fn show_next(self: &Self, tetro: &Tetromino) {
        const BASE_ROW: u16 = 4;
        const BASE_COL: u16 = 26;
        print!("{}", termion::cursor::Goto(BASE_COL, BASE_ROW));
        print!("┌────────┐");
        for row in 0..LAYOUT_HEIGHT {
            print!("{}│", termion::cursor::Goto(BASE_COL, BASE_ROW + (row as u16) + 1));
            for col in 0..LAYOUT_WIDTH {
                let shape = tetro.shape_at(&Coords{row: row, col: col});
                let color = convert_to_color(shape, false);
                print!("{}  {}", termion::color::Bg(color), termion::color::Bg(termion::color::Black));
            }
            print!("│");
        }
        print!("{}└────────┘", termion::cursor::Goto(BASE_COL, BASE_ROW + 5));
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
    }
}

fn convert_to_color(shape: Shape, is_active: bool) -> termion::color::AnsiValue {
    let shift = match is_active {
        true => 8,
        false => 0,
    };

    match shape {
        Shape::NoShape => termion::color::AnsiValue(0),
        Shape::OShape => termion::color::AnsiValue(1 + shift),
        Shape::IShape => termion::color::AnsiValue(2 + shift),
        Shape::TShape => termion::color::AnsiValue(3 + shift),
        Shape::JShape => termion::color::AnsiValue(4 + shift),
        Shape::LShape => termion::color::AnsiValue(5 + shift),
        Shape::SShape => termion::color::AnsiValue(6 + shift),
        Shape::ZShape => termion::color::AnsiValue(7 + shift),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playfield::Storage as Storage;
    use crate::figures::figures;
    use crate::playfield::Coords;
    use crate::playfield::HEIGHT as HEIGHT;

    #[test]
    fn show_empty_view() {
        let playfield: Playfield = Playfield::new(Storage::default());
        let view: ConsoleView = ConsoleView{};
        let active_tetro = FieldTetromino::default();

        for i in 0..HEIGHT {
            let row: String = view.show_row(&playfield, i, &active_tetro).iter().collect();
            assert_eq!(row, "          ");
        }
    }

    #[test]
    fn show_one_tetramino() {
        let mut playfield: Playfield = Playfield::new(Storage::default());
        let view: ConsoleView = ConsoleView{};
        let place_result = playfield.place(
            &figures::Tetromino::new(figures::Shape::OShape),
            Coords{col: 5, row: 2}
        );
        assert_eq!(place_result.is_ok(), true);
        let place_result = playfield.place(
            &figures::Tetromino::new(figures::Shape::TShape),
            Coords{col: 0, row: 5}
        );
        assert_eq!(place_result.is_ok(), true);
        let active_tetro = FieldTetromino{
            tetro: figures::Tetromino::new(figures::Shape::LShape),
            coords: Coords{col: 2, row: 4},
        };

        let result: [&'static str; HEIGHT as usize] = [
            "      oo  ",
            "      oo  ",
            "  L       ",
            " tLLL     ",
            "ttt       ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          "];

        for i in 0..HEIGHT {
            let row: String = view.show_row(&playfield, i, &active_tetro).iter().collect();
            assert_eq!(row, result[i as usize], "Line number {}", i);
        }
    }
}
