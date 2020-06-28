use crate::playfield::{Playfield, FieldTetromino, WIDTH, HEIGHT, Coords, ShapeAt, ShapeAtType};
use crate::figures::figures::{Shape, Tetromino, LAYOUT_HEIGHT, LAYOUT_WIDTH};
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
extern crate termion;

pub type Row = [char; WIDTH as usize];
pub enum ShowArgs<'a> {
    StaticArgs,
    PlayfieldArgs{playfield: &'a Playfield, active_tetro: &'a FieldTetromino, ghost_tetro: &'a FieldTetromino},
    ScoreArgs{level: i8, score: u32, lines: u32},
}

pub trait NewView {
    fn show_subview(self: &Self, args: &ShowArgs);
}
pub trait NextView {
    fn show_next(self: &Self, tetro: &mut Tetromino);
}
pub trait View: NewView + NextView {}
impl<T> View for T where T: NewView + NextView {}

pub struct ConsoleView {
}
const NEXT_TETRO_BASE_ROW: i8 = 4;
const NEXT_TETRO_BASE_COL: i8 = 26;
const SCORE_BASE_ROW: u16 = 2;
const SCORE_BASE_COL: u16 = 26;

impl NewView for ConsoleView {
    fn show_subview(self: &Self, args: &ShowArgs) {
        match args {
            ShowArgs::ScoreArgs{level, lines, score} => {
                print!("{}Level: {} Score: {} Lines: {}",
                       termion::cursor::Goto(SCORE_BASE_COL, SCORE_BASE_ROW), level, score, lines)
            },
            ShowArgs::StaticArgs => {
                print!("{}Move: ⬅️ ⬇️ ➡️  Rotate: ⬆️  Drop: Spacebar. Hold: h. Exit: q\n\r",
                       termion::cursor::Goto(1, 1));
                draw_rectangle(&Coords{row: 2, col: 1}, HEIGHT, WIDTH * 2);
                draw_rectangle(&Coords{row: NEXT_TETRO_BASE_ROW, col: NEXT_TETRO_BASE_COL}, LAYOUT_HEIGHT, LAYOUT_WIDTH * 2);
            },
            ShowArgs::PlayfieldArgs{playfield, active_tetro, ghost_tetro} => {
                for row in 0..HEIGHT {
                    print!("{}", termion::cursor::Goto(2, 3 + (row as u16)));
                    for col in 0..WIDTH {
                        let shape_at = playfield.shape_at(&Coords{row: HEIGHT - row - 1, col: col as i8}, active_tetro, ghost_tetro);
                        let color = convert_to_color(shape_at);
                        print!("{}  {}", termion::color::Bg(color), termion::color::Bg(termion::color::Black));
                    }
                }
                print!("{}", termion::color::Bg(termion::color::Black));
                print!("{}", termion::cursor::Goto(1, HEIGHT as u16 + 4));
            }
        };
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
    }
}

impl NextView for ConsoleView {
    fn show_next(self: &Self, tetro: &mut Tetromino) {
        for (coords, shape) in tetro {
            let color = convert_to_color(ShapeAt{shape: shape, shape_at_type: ShapeAtType::Static});
            print!("{}{}  {}", termion::cursor::Goto((NEXT_TETRO_BASE_COL + 1 + coords.col * 2) as u16,
                                                     (NEXT_TETRO_BASE_ROW + 1 + coords.row) as u16),
                               termion::color::Bg(color), termion::color::Bg(termion::color::Black));
        }
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
    }
}

fn draw_rectangle(top_left: &Coords, height: i8, width: i8) {
    print!("{}┌", termion::cursor::Goto(top_left.col as u16, top_left.row as u16));
    for _ in top_left.col..top_left.col + width {
        print!("─");
    }
    print!("┐");
    for r in top_left.row + 1..=top_left.row + height {
        print!("{}│{}│", termion::cursor::Goto(top_left.col as u16, r as u16),
                         termion::cursor::Goto((top_left.col + width + 1) as u16, r as u16));
    }
    print!("{}└", termion::cursor::Goto(top_left.col as u16, (top_left.row + height + 1) as u16));
    for _ in top_left.col..top_left.col + width {
        print!("─");
    }
    print!("┘");
}

struct ColorTable {
    shape: Shape,
    active_color: termion::color::AnsiValue,
    static_color: termion::color::AnsiValue,
}

macro_rules! rgb_color {
    ($r:expr,$g:expr,$b:expr) => {
        termion::color::AnsiValue(16 + 36 * $r + 6 * $g + $b);
    };
}

macro_rules! shape_and_color {
    ($shape:expr, $active:expr, $stat:expr) => {
        ColorTable{shape: $shape, active_color: $active, static_color: $stat}
    };
}

static COLOR_TABLE: &'static [ColorTable] = &[
    shape_and_color!(Shape::OShape, rgb_color!(5, 5, 0), rgb_color!(3, 3, 0)), // yellow
    shape_and_color!(Shape::IShape, rgb_color!(1, 5, 5), rgb_color!(0, 3, 3)), // cyan
    shape_and_color!(Shape::TShape, rgb_color!(5, 0, 5), rgb_color!(2, 0, 2)), // purple
    shape_and_color!(Shape::SShape, rgb_color!(0, 5, 0), rgb_color!(0, 2, 0)), // green
    shape_and_color!(Shape::ZShape, rgb_color!(5, 0, 0), rgb_color!(2, 0, 0)), // red
    shape_and_color!(Shape::JShape, rgb_color!(0, 0, 5), rgb_color!(0, 0, 3)), // blue
    shape_and_color!(Shape::LShape, rgb_color!(5, 2, 0), rgb_color!(3, 1, 0)), // orange
];

fn convert_to_color(shape_at: ShapeAt) -> termion::color::AnsiValue {
    if shape_at.shape_at_type == ShapeAtType::Ghost {
        return termion::color::AnsiValue::grayscale(3);
    }

    for c in COLOR_TABLE {
        if c.shape == shape_at.shape {
            if shape_at.shape_at_type == ShapeAtType::Active {
                return c.active_color;
            } else {
                return c.static_color;
            }
        }
    }

    /* black for every NoShape */
    termion::color::AnsiValue(0)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::playfield::Storage as Storage;
    // use crate::figures::figures;
    // use crate::playfield::Coords;
    // use crate::playfield::HEIGHT as HEIGHT;

    // #[test]
    // fn show_empty_view() {
    //     let playfield: Playfield = Playfield::new(Storage::default());
    //     let view: ConsoleView = ConsoleView{};
    //     let active_tetro = FieldTetromino::default();

    //     for i in 0..HEIGHT {
    //         let row: String = view.show_row(&playfield, i, &active_tetro).iter().collect();
    //         assert_eq!(row, "          ");
    //     }
    // }

    // #[test]
    // fn show_one_tetramino() {
    //     let mut playfield: Playfield = Playfield::new(Storage::default());
    //     let view: ConsoleView = ConsoleView{};
    //     let place_result = playfield.place(
    //         &figures::Tetromino::new(figures::Shape::OShape),
    //         Coords{col: 5, row: 2}
    //     );
    //     assert_eq!(place_result.is_ok(), true);
    //     let place_result = playfield.place(
    //         &figures::Tetromino::new(figures::Shape::TShape),
    //         Coords{col: 0, row: 5}
    //     );
    //     assert_eq!(place_result.is_ok(), true);
    //     let active_tetro = FieldTetromino{
    //         tetro: figures::Tetromino::new(figures::Shape::LShape),
    //         coords: Coords{col: 2, row: 4},
    //     };

    //     let result: [&'static str; HEIGHT as usize] = [
    //         "      oo  ",
    //         "      oo  ",
    //         "  L       ",
    //         " tLLL     ",
    //         "ttt       ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          ",
    //         "          "];

    //     for i in 0..HEIGHT {
    //         let row: String = view.show_row(&playfield, i, &active_tetro).iter().collect();
    //         assert_eq!(row, result[i as usize], "Line number {}", i);
    //     }
    // }
}
