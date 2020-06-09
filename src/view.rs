use crate::playfield::Playfield;
use crate::playfield::WIDTH as WIDTH;
use crate::playfield::HEIGHT as HEIGHT;
use crate::figures::figures::Shape as Shape;

pub struct View {
    playfield: Playfield,
}

pub type Row = [char; WIDTH as usize];

impl View {
    pub fn new(playfield: Playfield) -> View {
        View{playfield: playfield}
    }

    pub fn show_row(self: &Self, row: i8) -> Row {
        let mut result: Row = [' '; WIDTH as usize];
        for (i, s) in self.playfield.shape_row(row).iter().enumerate() {
            let value = match s {
                Shape::NoShape => ' ',
                Shape::OShape => 'o',
                Shape::IShape => 'i',
                Shape::TShape => 't',
                Shape::JShape => 'j',
                Shape::LShape => 'l',
                Shape::SShape => 's',
                Shape::ZShape => 'z',
            };
            result[i] = value;
        }
        result
    }

    pub fn show_rows(self: &Self) -> [Row; HEIGHT as usize] {
        let mut result: [Row; HEIGHT as usize] = [[' '; WIDTH as usize]; HEIGHT as usize];

        for i in 0..HEIGHT {
            result[i as usize] = self.show_row(i as i8);
        }

        result
    }
    // pub fn rows(self: &Self) -> impl Iterator<Item = &Row> {
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playfield::Storage as Storage;
    use crate::figures::figures;
    use crate::playfield::Coords;

    #[test]
    fn show_empty_view() {
        let storage: Storage = Default::default();
        let playfield: Playfield = Playfield::new(storage);
        let view: View = View::new(playfield);

        for i in 0..HEIGHT {
            let row: String = view.show_row(i).iter().collect();
            assert_eq!(row, "          ");
        }
    }

    #[test]
    fn show_one_tetramino() {
        let storage: Storage = Default::default();
        let playfield: Playfield = Playfield::new(storage);
        let mut view: View = View::new(playfield);
        let place_result = view.playfield.place(
            &figures::Tetromino::new(figures::Shape::OShape),
            Coords{col: 5, row: 2}
        );
        assert_eq!(place_result.is_ok(), true);
        let place_result = view.playfield.place(
            &figures::Tetromino::new(figures::Shape::TShape),
            Coords{col: 0, row: 5}
        );
        assert_eq!(place_result.is_ok(), true);
        let place_result = view.playfield.place(
            &figures::Tetromino::new(figures::Shape::LShape),
            Coords{col: 2, row: 4}
        );
        assert_eq!(place_result.is_ok(), true);

        let result: [&'static str; HEIGHT as usize] = [
            "      oo  ",
            "      oo  ",
            "  l       ",
            " tlll     ",
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
            let row: String = view.show_row(i).iter().collect();
            assert_eq!(row, result[i as usize], "Line number {}", i);
        }
    }
}
