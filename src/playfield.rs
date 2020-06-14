use crate::figures::*;

pub const WIDTH: i8 = 10;
pub const HEIGHT: i8 = 20;
const TOTAL_HEIGHT: i8 = 30;

type PlayfieldStorage = [[figures::Shape; WIDTH as usize]; TOTAL_HEIGHT as usize];

pub struct Storage {
    playfield: PlayfieldStorage,
    active_layout: figures::Layout,
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            playfield: Default::default(),
            active_layout: Default::default(),
        }
    }
}

pub struct ActiveTetromino {
    pub shape: figures::Shape,
    pub coords: Coords,
}

impl Default for ActiveTetromino {
    fn default() -> Self {
        ActiveTetromino{
            shape: figures::Shape::NoShape,
            coords: Coords{row: 0, col: 0}
        }
    }
}

pub struct Playfield {
    pub storage: Storage,
    active_tetro: ActiveTetromino,
}

pub struct OutOfBoundsError;

#[derive(Copy, Clone)]
pub struct Coords {
    pub row: i8,
    pub col: i8,
}

impl Playfield {
    pub fn place(self: &mut Self, figure: &figures::Tetromino, coords: Coords) -> Result<(), OutOfBoundsError> {
        if self.can_place(figure, &coords) {
            for row in 0..figure.layout.len() {
                for col in 0..figure.layout.len() {
                    if figure.layout[row][col] != 0 {
                        // already checked that it can be placed, so just place it
                        // rows are counted from bottom to top, so index should be reverted
                        self.storage.playfield[(coords.row as usize) - row][(coords.col as usize) + col] = figure.shape;
                    }
                }
            }

            Ok(())
        } else {
            Err(OutOfBoundsError{})
        }
    }

    pub fn can_place(self: &mut Self, figure: &figures::Tetromino, coords: &Coords) -> bool {
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

    pub fn shape_at(self: &Self, coords: &Coords) -> figures::Shape {
        if coords.col < 0 || coords.col > WIDTH || coords.row < 0 || coords.row > HEIGHT {
            figures::Shape::NoShape
        } else {
            let (inside, active_coords) = self.inside_active_tetro(&coords);
            if inside && (self.storage.active_layout[active_coords.row as usize][active_coords.col as usize] == 1) {
                self.active_tetro.shape
            } else {
                self.storage.playfield[coords.row as usize][coords.col as usize]
            }
        }
    }

    pub fn new_active(self: &mut Self, shape: figures::Shape, coords: &Coords) -> Result<(&ActiveTetromino), OutOfBoundsError> {
        let figure = figures::Tetromino::new(shape);

        if self.can_place(&figure, &coords) {
            self.active_tetro.coords = *coords;
            self.active_tetro.shape = shape;
            self.storage.active_layout = figure.layout;
            Ok(&self.active_tetro)
        } else {
            Err(OutOfBoundsError{})
        }
    }

    pub fn place_active(self: &mut Self) -> Result<(), OutOfBoundsError> {
        self.place(
            &figures::Tetromino{shape: self.active_tetro.shape, layout: self.storage.active_layout},
            self.active_tetro.coords
        )
    }

    fn inside_active_tetro(self: &Self, coords: &Coords) -> (bool, Coords) {
        /* horizontal match */
        if coords.col >= self.active_tetro.coords.col
                && coords.col < self.active_tetro.coords.col + figures::LAYOUT_WIDTH
                && coords.row <= self.active_tetro.coords.row
                && coords.row > self.active_tetro.coords.row - figures::LAYOUT_HEIGHT {
            (true, Coords{row: self.active_tetro.coords.row - coords.row,
                col: coords.col - self.active_tetro.coords.col})
        } else {
            (false, Coords{row: 0, col: 0})
        }
    }

    pub fn new(storage: Storage) -> Playfield {
        Playfield{storage: storage, active_tetro: ActiveTetromino::default()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_at_in_and_outside_bounds() {
        let playfield: Playfield = Playfield::new(Default::default());

        assert_eq!(playfield.shape_at(&Coords{col: 0, row: 0}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: WIDTH + 1, row: HEIGHT + 1}), figures::Shape::NoShape);
    }

    #[test]
    fn shape_at_after_placing_shape() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let tetro = figures::Tetromino::new(figures::Shape::OShape);
        let place_coords = Coords{col: 5, row: 10};

        let place_result = playfield.place(&tetro, place_coords);
        assert_eq!(place_result.is_ok(), true);

        // check for every position that shape is either OShape or None
        assert_eq!(playfield.shape_at(&Coords{..place_coords}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col, row: place_coords.row - 1}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row - 1}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row - 1}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row - 1}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col, row: place_coords.row - 2}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row - 2}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row - 2}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row - 2}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col, row: place_coords.row - 3}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row - 3}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row - 3}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row - 3}), figures::Shape::NoShape);
    }

    #[test]
    fn shape_at_after_new_active_shape() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let create_coords = Coords{col: 5, row: 10};
        let create_result = playfield.new_active(figures::Shape::OShape, &create_coords);
        assert_eq!(create_result.is_ok(), true);

        // check for every position that shape is either OShape or None
        assert_eq!(playfield.shape_at(&Coords{..create_coords}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col, row: create_coords.row - 1}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row - 1}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row - 1}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row - 1}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col, row: create_coords.row - 2}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row - 2}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row - 2}), figures::Shape::OShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row - 2}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col, row: create_coords.row - 3}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row - 3}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row - 3}), figures::Shape::NoShape);
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row - 3}), figures::Shape::NoShape);
    }

    #[test]
    fn place_failure_outside_bounds() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let tetro = figures::Tetromino::new(figures::Shape::OShape);

        // doesn't fit vertically
        assert_eq!(playfield.can_place(&tetro, &Coords{col: 0, row: 0}), false);
        // doesn't fit horizontally
        assert_eq!(playfield.can_place(&tetro, &Coords{col: WIDTH - 1, row: 5}), false);
    }
}
