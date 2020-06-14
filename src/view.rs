use crate::playfield::Playfield;
use crate::playfield::WIDTH as WIDTH;
use crate::playfield::Coords as Coords;
use crate::figures::figures::Shape as Shape;

pub type Row = [char; WIDTH as usize];
pub struct ConsoleView {
}

pub trait View {
    fn show_row(&self, playfield: &Playfield, row: i8) -> Row;
    /* TODO: add rows iterator */
}

impl View for ConsoleView {
    fn show_row(&self, playfield: &Playfield, row: i8) -> Row {
        let mut result: Row = [' '; WIDTH as usize];
        for i in 0..WIDTH as usize {
            let (shape, is_active) = playfield.shape_at(&Coords{row: row, col: i as i8});
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

        for i in 0..HEIGHT {
            let row: String = view.show_row(&playfield, i).iter().collect();
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
        let place_result = playfield.new_active(figures::Shape::LShape, &Coords{col: 2, row: 4});
        assert_eq!(place_result.is_ok(), true);

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
            let row: String = view.show_row(&playfield, i).iter().collect();
            assert_eq!(row, result[i as usize], "Line number {}", i);
        }
    }
}
