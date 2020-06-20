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

pub enum Dir {
    Down,
    Left,
    Right,
    Rotate,
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
                        self.storage.playfield[(coords.row - row as i8) as usize][(coords.col + col as i8) as usize] = figure.shape;
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
                    if coords.col + (col as i8) >= WIDTH || coords.col + (col as i8) < 0 {
                        return false;
                    }
                    if self.storage.playfield[(coords.row - row as i8) as usize][(coords.col + col as i8) as usize] != figures::Shape::NoShape {
                        return false;
                    }
                }
            }
        };

        true
    }

    /**
     * \brief Get shape stored at given coordinates.
     *
     * \param self Reference to playfield instance.
     * \param coords Coordinates to check.
     *
     * \return A tuple. First item is shape for given coordinate or figures::Shape::NoShape.
     * Second item is true if given shape belongs to active tetromino.
     */
    pub fn shape_at(self: &Self, coords: &Coords) -> (figures::Shape, bool) {
        if coords.col < 0 || coords.col > WIDTH || coords.row < 0 || coords.row > HEIGHT {
            (figures::Shape::NoShape, false)
        } else {
            let (inside, active_coords) = self.inside_active_tetro(&coords);
            if inside && (self.storage.active_layout[active_coords.row as usize][active_coords.col as usize] == 1) {
                (self.active_tetro.shape, true)
            } else {
                (self.storage.playfield[coords.row as usize][coords.col as usize], false)
            }
        }
    }

    pub fn new_active(self: &mut Self, tetro: &figures::Tetromino, coords: &Coords) -> Result<(), OutOfBoundsError> {
        if self.can_place(tetro, &coords) {
            self.active_tetro.coords = *coords;
            self.active_tetro.shape = tetro.shape;
            self.storage.active_layout = tetro.layout;
            Ok(())
        } else {
            Err(OutOfBoundsError{})
        }
    }

    pub fn place_active(self: &mut Self) -> Result<(), OutOfBoundsError> {
        let result = self.place(
            &figures::Tetromino{shape: self.active_tetro.shape, layout: self.storage.active_layout},
            self.active_tetro.coords
        );

        /* active is no longer active */
        self.active_tetro = Default::default();

        result
    }

    pub fn move_active(self: &mut Self, dir: Dir) -> bool {
        let new_coords = match dir {
            Dir::Down => Coords{col: self.active_tetro.coords.col, row: self.active_tetro.coords.row - 1},
            Dir::Left => Coords{col: self.active_tetro.coords.col - 1, row: self.active_tetro.coords.row},
            Dir::Right => Coords{col: self.active_tetro.coords.col + 1, row: self.active_tetro.coords.row},
            _ => self.active_tetro.coords,
        };
        let figure = figures::Tetromino{shape: self.active_tetro.shape, layout: self.storage.active_layout};

        if self.can_place(&figure, &new_coords) {
            self.active_tetro.coords = new_coords;
            return true;
        }

        false
    }

    pub fn turn_active(self: &mut Self) -> bool {
        let mut tetro = figures::Tetromino{shape: self.active_tetro.shape, layout: self.storage.active_layout};
        figures::rotate(&mut tetro);
        let coords = self.active_tetro.coords;

        if self.can_place(&tetro, &coords) {
            self.storage.active_layout = tetro.layout;
            return true;
        }

        false
    }

    pub fn delete_row(self: &mut Self, row: i8) {
        if row < 0 || row > TOTAL_HEIGHT {
            return;
        }

        for r in row..TOTAL_HEIGHT-1 {
            for c in 0..(WIDTH as usize) {
                self.storage.playfield[r as usize][c] = self.storage.playfield[(r + 1) as usize][c];
            }
        }
    }

    fn inside_active_tetro(self: &Self, coords: &Coords) -> (bool, Coords) {
        let not_found = (false, Coords{row: 0, col: 0});
        if self.active_tetro.shape == figures::Shape::NoShape {
            /* no active shape */
            return not_found;
        }
        if coords.col < self.active_tetro.coords.col ||
                coords.col >= self.active_tetro.coords.col + figures::LAYOUT_WIDTH {
            /* no horizontal match */
            return not_found;
        }
        if coords.row > self.active_tetro.coords.row ||
                coords.row <= self.active_tetro.coords.row - figures::LAYOUT_HEIGHT {
            /* no vertical match */
            return not_found;
        }

        (true, Coords{row: self.active_tetro.coords.row - coords.row,
                      col: coords.col - self.active_tetro.coords.col})
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

        assert_eq!(playfield.shape_at(&Coords{col: 0, row: 0}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: WIDTH + 1, row: HEIGHT + 1}), (figures::Shape::NoShape, false));
    }

    #[test]
    fn shape_at_after_placing_shape() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let tetro = figures::Tetromino::new(figures::Shape::OShape);
        let place_coords = Coords{col: 5, row: 10};

        let place_result = playfield.place(&tetro, place_coords);
        assert_eq!(place_result.is_ok(), true);

        // check for every position that shape is either OShape or None
        assert_eq!(playfield.shape_at(&Coords{..place_coords}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col, row: place_coords.row - 1}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row - 1}), (figures::Shape::OShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row - 1}), (figures::Shape::OShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row - 1}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col, row: place_coords.row - 2}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row - 2}), (figures::Shape::OShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row - 2}), (figures::Shape::OShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row - 2}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col, row: place_coords.row - 3}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row - 3}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row - 3}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row - 3}), (figures::Shape::NoShape, false));
    }

    #[test]
    fn shape_at_after_new_active_shape() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let create_coords = Coords{col: 5, row: 10};
        let tetro = figures::Tetromino::new(figures::Shape::OShape);
        let create_result = playfield.new_active(&tetro, &create_coords);
        assert_eq!(create_result.is_ok(), true);

        // check for every position that shape is either OShape or None
        assert_eq!(playfield.shape_at(&Coords{..create_coords}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col, row: create_coords.row - 1}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row - 1}), (figures::Shape::OShape, true));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row - 1}), (figures::Shape::OShape, true));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row - 1}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col, row: create_coords.row - 2}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row - 2}), (figures::Shape::OShape, true));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row - 2}), (figures::Shape::OShape, true));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row - 2}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col, row: create_coords.row - 3}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row - 3}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row - 3}), (figures::Shape::NoShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row - 3}), (figures::Shape::NoShape, false));
    }

    #[test]
    fn bounce_left_wall() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let create_coords = Coords{col: -1, row: 10};
        let tetro = figures::Tetromino::new(figures::Shape::OShape);
        let create_result = playfield.new_active(&tetro, &create_coords);
        assert_eq!(create_result.is_ok(), true);
        let move_result = playfield.move_active(Dir::Left);
        assert_eq!(move_result, false);
    }

    #[test]
    fn bounce_right_wall() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let create_coords = Coords{col: WIDTH - 3, row: 10};
        let tetro = figures::Tetromino::new(figures::Shape::OShape);
        let create_result = playfield.new_active(&tetro, &create_coords);
        assert_eq!(create_result.is_ok(), true);
        let move_result = playfield.move_active(Dir::Right);
        assert_eq!(move_result, false);
    }

    #[test]
    fn o_in_bottom_left_corner() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let create_coords = Coords{col: -1, row: 2};
        let tetro = figures::Tetromino::new(figures::Shape::OShape);
        let create_result = playfield.new_active(&tetro, &create_coords);
        assert_eq!(create_result.is_ok(), true);
        let move_result = playfield.move_active(Dir::Down);
        assert_eq!(move_result, false);
        let place_result = playfield.place_active();
        assert_eq!(place_result.is_ok(), true);
    }


    #[test]
    fn lshape_in_bottom_left_corner() {
        /* prepare a figure like this:
         * |  L       |
         * |LLL       |
         * +----------+
         */
        let mut playfield: Playfield = Playfield::new(Default::default());
        let tetro = figures::Tetromino::new(figures::Shape::LShape);
        let create_result = playfield.new_active(&tetro, &Coords{col: 0, row: 2});
        assert_eq!(create_result.is_ok(), true);
        assert_eq!(playfield.turn_active(), true);
        assert_eq!(playfield.turn_active(), true);
        playfield.move_active(Dir::Down);
        playfield.move_active(Dir::Down);
        /* and place this figure */
        let place_result = playfield.place_active();
        assert_eq!(place_result.is_ok(), true);

        /* verify that the bottom row is shown properly */
        assert_eq!(playfield.shape_at(&Coords{col: 0, row: 0}), (figures::Shape::LShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: 1, row: 0}), (figures::Shape::LShape, false));
        assert_eq!(playfield.shape_at(&Coords{col: 2, row: 0}), (figures::Shape::LShape, false));
    }

    #[test]
    fn down_on_top_of_placed_tetro() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let place_result = playfield.place(
            &figures::Tetromino::new(figures::Shape::OShape),
            Coords{col: -1, row: 2}
        );
        assert_eq!(place_result.is_ok(), true);
        let create_result = playfield.new_active(
            &figures::Tetromino::new(figures::Shape::OShape),
            &Coords{col: -1, row: 4}
        );
        assert_eq!(create_result.is_ok(), true);
        let move_result = playfield.move_active(Dir::Down);
        assert_eq!(move_result, false);
        let place_result = playfield.place_active();
        assert_eq!(place_result.is_ok(), true);
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
