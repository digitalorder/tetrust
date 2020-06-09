use crate::figures::*;

const WIDTH: i8 = 10;
const HEIGHT: i8 = 20;
const TOTAL_HEIGHT: i8 = 30;

pub type Storage = [[figures::Shape; WIDTH as usize]; TOTAL_HEIGHT as usize];

pub struct Playfield {
    storage: Storage,
}

pub struct OutOfBoundsError;

#[derive(Copy, Clone)]
pub struct Coords {
    row: i8,
    col: i8,
}

impl Playfield {
    pub fn place(self: &mut Self, figure: &figures::Tetromino, coords: Coords) -> Result<(), OutOfBoundsError> {
        if self.can_place(figure, coords) {
            for row in 0..figure.layout.len() {
                for col in 0..figure.layout.len() {
                    if figure.layout[row][col] != 0 {
                        // already checked that it can be placed, so just place it
                        // rows are counted from bottom to top, so index should be reverted
                        self.storage[(coords.row as usize) - row][(coords.col as usize) + col] = figure.shape;
                    }
                }
            }

            Ok(())
        } else {
            Err(OutOfBoundsError{})
        }
    }

    pub fn can_place(self: &mut Self, figure: &figures::Tetromino, coords: Coords) -> bool {
        for row in 0..figure.layout.len() {
            for col in 0..figure.layout.len() {
                if figure.layout[row][col] != 0 {
                    if row as i8 > coords.row || coords.row >= row as i8 + TOTAL_HEIGHT {
                        return false;
                    }
                    if coords.col + col as i8 >= WIDTH {
                        return false;
                    }
                }
            }
        };

        true
    }

    pub fn shape_at(self: &Self, coords: Coords) -> figures::Shape {
        if coords.col < 0 || coords.col > WIDTH || coords.row < 0 || coords.row > HEIGHT {
            figures::Shape::NoShape
        } else {
            self.storage[coords.row as usize][coords.col as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_at_in_and_outside_bounds() {
        let storage: Storage = Default::default();
        let playfield: Playfield = Playfield{storage: storage};

        assert_eq!(playfield.shape_at(Coords{col: 0, row: 0}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: WIDTH + 1, row: HEIGHT + 1}), figures::Shape::NoShape);
    }

    #[test]
    fn shape_at_after_placing_shape() {
        let storage: Storage = Default::default();
        let mut playfield: Playfield = Playfield{storage: storage};
        let tetro = figures::Tetromino::new(figures::Shape::OShape);
        let place_coords = Coords{col: 5, row: 10};

        let place_result = playfield.place(&tetro, place_coords);
        assert_eq!(place_result.is_ok(), true);

        // check for every position that shape is either OShape or None
        assert_eq!(playfield.shape_at(Coords{..place_coords}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 1, row: place_coords.row}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 2, row: place_coords.row}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 3, row: place_coords.row}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col, row: place_coords.row - 1}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 1, row: place_coords.row - 1}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 2, row: place_coords.row - 1}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 3, row: place_coords.row - 1}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col, row: place_coords.row - 2}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 1, row: place_coords.row - 2}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 2, row: place_coords.row - 2}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 3, row: place_coords.row - 2}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col, row: place_coords.row - 3}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 1, row: place_coords.row - 3}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 2, row: place_coords.row - 3}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(Coords{col: place_coords.col + 3, row: place_coords.row - 3}), figures::Shape::NoShape);
    }

    #[test]
    fn place_failure_outside_bounds() {
        let storage: Storage = Default::default();
        let mut playfield: Playfield = Playfield{storage: storage};
        let tetro = figures::Tetromino::new(figures::Shape::OShape);

        // doesn't fit vertically
        assert_eq!(playfield.can_place(&tetro, Coords{col: 0, row: 0}), false);
        // doesn't fit horizontally
        assert_eq!(playfield.can_place(&tetro, Coords{col: WIDTH - 1, row: 5}), false);
    }
}
