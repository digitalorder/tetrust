use crate::figures::*;

pub const WIDTH: i8 = 10;
pub const HEIGHT: i8 = 20;
const TOTAL_HEIGHT: i8 = 30;

type PlayfieldStorage = [[figures::Shape; WIDTH as usize]; TOTAL_HEIGHT as usize];

pub struct Storage {
    playfield: PlayfieldStorage,
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            playfield: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct FieldTetromino {
    pub coords: Coords,
    pub tetro: figures::Tetromino,
}

impl Default for FieldTetromino {
    fn default() -> Self {
        FieldTetromino{
            tetro: figures::Tetromino::new(figures::Shape::NoShape),
            coords: Coords{row: 0, col: 0},
        }
    }
}

pub enum Dir {
    Down,
    Left,
    Right,
    Rotate,
}

#[derive(Debug, PartialEq)]
pub enum ShapeAtType {
    Static,
    Active,
    Ghost,
}

#[derive(Debug, PartialEq)]
pub struct ShapeAt {
    pub shape_at_type: ShapeAtType,
    pub shape: figures::Shape,
}

pub struct Playfield {
    storage: Storage,
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
            for row in 0..figures::LAYOUT_WIDTH {
                for col in 0..figures::LAYOUT_HEIGHT {
                    /* index may go out of bounds for empty parts of figure,
                     * so we need to check for that */
                    if coords.row < row || coords.col + col < 0 {
                        continue;
                    }

                    let shape = figure.shape_at(&Coords{row: row, col: col});

                    if shape != figures::Shape::NoShape {
                        self.storage.playfield[(coords.row - row) as usize][(coords.col + col) as usize] = shape;
                    }
                }
            }

            Ok(())
        } else {
            Err(OutOfBoundsError{})
        }
    }

    pub fn can_place(self: &Self, figure: &figures::Tetromino, coords: &Coords) -> bool {
        for row in 0..figures::LAYOUT_WIDTH {
            for col in 0..figures::LAYOUT_HEIGHT {
                if figure.shape_at(&Coords{row: row, col: col}) != figures::Shape::NoShape {
                    if row > coords.row || coords.row >= row + TOTAL_HEIGHT {
                        return false;
                    }
                    if coords.col + col >= WIDTH || coords.col + col < 0 {
                        return false;
                    }
                    if self.storage.playfield[(coords.row - row) as usize][(coords.col + col) as usize] != figures::Shape::NoShape {
                        return false;
                    }
                }
            }
        };

        true
    }

    pub fn row_filled(self: &Self, row: i8) -> bool {
        for col in 0..WIDTH {
            if self.storage.playfield[row as usize][col as usize] == figures::Shape::NoShape {
                return false;
            }
        }

        true
    }

    /**
     * \brief Get shape stored at given coordinates.
     *
     * \param self Reference to playfield instance.
     * \param active_tetro Tetro with coordinates existing on playfield.
     *
     * \return A tuple. First item is shape for given coordinate or figures::Shape::NoShape.
     * Second item is true if given shape belongs to active tetromino.
     */
    pub fn shape_at(self: &Self, coords: &Coords, active_tetro: &FieldTetromino, ghost_tetro: &FieldTetromino) -> ShapeAt {
        if coords.col < 0 || coords.col > WIDTH || coords.row < 0 || coords.row > HEIGHT {
            ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static}
        } else {
            let (inside_active, active_coords) = if active_tetro.tetro.shape == figures::Shape::NoShape {
                (false, Coords{row: 0, col: 0})
            } else {
                Playfield::inside_tetro_coords(&coords, &active_tetro.coords)
            };

            let (inside_ghost, ghost_coords) = if ghost_tetro.tetro.shape == figures::Shape::NoShape {
                (false, Coords{row: 0, col: 0})
            } else {
                Playfield::inside_tetro_coords(&coords, &ghost_tetro.coords)
            };

            if inside_active && active_tetro.tetro.shape_at(&active_coords) != figures::Shape::NoShape {
                return ShapeAt{shape: active_tetro.tetro.shape, shape_at_type: ShapeAtType::Active};
            }

            if inside_ghost && ghost_tetro.tetro.shape_at(&ghost_coords) != figures::Shape::NoShape {
                return ShapeAt{shape: active_tetro.tetro.shape, shape_at_type: ShapeAtType::Ghost};
            }

            return ShapeAt{shape: self.storage.playfield[coords.row as usize][coords.col as usize], shape_at_type: ShapeAtType::Static}
        }
    }

    pub fn move_tetro(self: &Self, tetro: &mut FieldTetromino, dir: Dir) -> bool {
        if tetro.tetro.shape == figures::Shape::NoShape {
            return false;
        }

        let new_coords = match dir {
            Dir::Down => Coords{col: tetro.coords.col, row: tetro.coords.row - 1},
            Dir::Left => Coords{col: tetro.coords.col - 1, row: tetro.coords.row},
            Dir::Right => Coords{col: tetro.coords.col + 1, row: tetro.coords.row},
            _ => return false,
        };

        if self.can_place(&tetro.tetro, &new_coords) {
            tetro.coords = new_coords;
            return true;
        }

        false
    }

    pub fn turn_tetro(self: &Self, tetro: &mut FieldTetromino) -> bool {
        if tetro.tetro.shape == figures::Shape::NoShape {
            return false;
        }

        let mut turned_tetro = *tetro;
        figures::rotate(&mut turned_tetro.tetro);

        if self.can_place(&turned_tetro.tetro, &turned_tetro.coords) {
            *tetro = turned_tetro;
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

    fn inside_tetro_coords(coords: &Coords, tetro_coords: &Coords) -> (bool, Coords) {
        let outside_bounds = (false, Coords{row: 0, col: 0});

        if coords.col < tetro_coords.col ||
                coords.col >= tetro_coords.col + figures::LAYOUT_WIDTH {
            /* no horizontal match */
            return outside_bounds;
        }
        if coords.row > tetro_coords.row ||
                coords.row <= tetro_coords.row - figures::LAYOUT_HEIGHT {
            /* no vertical match */
            return outside_bounds;
        }

        (true, Coords{row: tetro_coords.row - coords.row,
                      col: coords.col - tetro_coords.col})
    }

    pub fn new(storage: Storage) -> Playfield {
        Playfield{storage: storage}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_at_in_and_outside_bounds() {
        let playfield: Playfield = Playfield::new(Default::default());
        let active_tetro = FieldTetromino::default();

        assert_eq!(playfield.shape_at(&Coords{col: 0, row: 0}, &active_tetro, &active_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: WIDTH + 1, row: HEIGHT + 1}, &active_tetro, &active_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
    }

    #[test]
    fn shape_at_after_placing_shape() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let tetro = figures::Tetromino::new(figures::Shape::OShape);
        let active_tetro = FieldTetromino::default();
        let ghost_tetro = active_tetro;
        let place_coords = Coords{col: 5, row: 10};

        let place_result = playfield.place(&tetro, place_coords);
        assert_eq!(place_result.is_ok(), true);

        // check for every position that shape is either OShape or None
        assert_eq!(playfield.shape_at(&Coords{..place_coords}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col, row: place_coords.row - 1}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row - 1}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::OShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row - 1}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::OShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row - 1}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col, row: place_coords.row - 2}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row - 2}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::OShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row - 2}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::OShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row - 2}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col, row: place_coords.row - 3}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 1, row: place_coords.row - 3}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 2, row: place_coords.row - 3}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: place_coords.col + 3, row: place_coords.row - 3}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
    }

    #[test]
    fn shape_at_after_new_active_shape() {
        let playfield: Playfield = Playfield::new(Default::default());
        let create_coords = Coords{col: 5, row: 10};
        let active_tetro = FieldTetromino{
            tetro: figures::Tetromino::new(figures::Shape::OShape),
            coords: create_coords,
        };
        let ghost_tetro = active_tetro;

        // check for every position that shape is either OShape or None
        assert_eq!(playfield.shape_at(&Coords{..create_coords}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col, row: create_coords.row - 1}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row - 1}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::OShape, shape_at_type: ShapeAtType::Active});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row - 1}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::OShape, shape_at_type: ShapeAtType::Active});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row - 1}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col, row: create_coords.row - 2}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row - 2}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::OShape, shape_at_type: ShapeAtType::Active});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row - 2}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::OShape, shape_at_type: ShapeAtType::Active});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row - 2}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col, row: create_coords.row - 3}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 1, row: create_coords.row - 3}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 2, row: create_coords.row - 3}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: create_coords.col + 3, row: create_coords.row - 3}, &active_tetro, &ghost_tetro),
                   ShapeAt{shape: figures::Shape::NoShape, shape_at_type: ShapeAtType::Static});
    }

    #[test]
    fn bounce_left_wall() {
        let playfield: Playfield = Playfield::new(Default::default());
        let mut active_tetro = FieldTetromino{
            tetro: figures::Tetromino::new(figures::Shape::OShape),
            coords: Coords{col: -1, row: 10},
        };
        let move_result = playfield.move_tetro(&mut active_tetro, Dir::Left);
        assert_eq!(move_result, false);
    }

    #[test]
    fn bounce_right_wall() {
        let playfield: Playfield = Playfield::new(Default::default());
        let mut active_tetro = FieldTetromino{
            tetro: figures::Tetromino::new(figures::Shape::OShape),
            coords: Coords{col: WIDTH - 3, row: 10}
        };
        let move_result = playfield.move_tetro(&mut active_tetro, Dir::Right);
        assert_eq!(move_result, false);
    }

    #[test]
    fn o_in_bottom_left_corner() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let mut active_tetro = FieldTetromino{
            tetro: figures::Tetromino::new(figures::Shape::OShape),
            coords: Coords{col: -1, row: 2},
        };
        let move_result = playfield.move_tetro(&mut active_tetro, Dir::Down);
        assert_eq!(move_result, false);
        let place_result = playfield.place(&active_tetro.tetro, active_tetro.coords);
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
        let mut active_tetro = FieldTetromino{
            tetro: figures::Tetromino::new(figures::Shape::LShape),
            coords: Coords{col: 0, row: 2},
        };
        assert_eq!(playfield.turn_tetro(&mut active_tetro), true);
        assert_eq!(playfield.turn_tetro(&mut active_tetro), true);
        playfield.move_tetro(&mut active_tetro, Dir::Down);
        playfield.move_tetro(&mut active_tetro, Dir::Down);
        /* and place this figure */
        let place_result = playfield.place(&active_tetro.tetro, active_tetro.coords);
        assert_eq!(place_result.is_ok(), true);

        /* verify that the bottom row is shown properly */
        assert_eq!(playfield.shape_at(&Coords{col: 0, row: 0}, &FieldTetromino::default(), &FieldTetromino::default()),
                   ShapeAt{shape: figures::Shape::LShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: 1, row: 0}, &FieldTetromino::default(), &FieldTetromino::default()),
                   ShapeAt{shape: figures::Shape::LShape, shape_at_type: ShapeAtType::Static});
        assert_eq!(playfield.shape_at(&Coords{col: 2, row: 0}, &FieldTetromino::default(), &FieldTetromino::default()),
                   ShapeAt{shape: figures::Shape::LShape, shape_at_type: ShapeAtType::Static});
    }

    #[test]
    fn down_on_top_of_placed_tetro() {
        let mut playfield: Playfield = Playfield::new(Default::default());
        let place_result = playfield.place(
            &figures::Tetromino::new(figures::Shape::OShape),
            Coords{col: -1, row: 2}
        );
        assert_eq!(place_result.is_ok(), true);
        let mut active_tetro = FieldTetromino{
            tetro: figures::Tetromino::new(figures::Shape::OShape),
            coords: Coords{col: -1, row: 4},
        };
        let move_result = playfield.move_tetro(&mut active_tetro, Dir::Down);
        assert_eq!(move_result, false);
        let place_result = playfield.place(&active_tetro.tetro, active_tetro.coords);
        assert_eq!(place_result.is_ok(), true);
    }

    #[test]
    fn place_failure_outside_bounds() {
        let playfield: Playfield = Playfield::new(Default::default());
        let tetro = figures::Tetromino::new(figures::Shape::OShape);

        // doesn't fit vertically
        assert_eq!(playfield.can_place(&tetro, &Coords{col: 0, row: 0}), false);
        // doesn't fit horizontally
        assert_eq!(playfield.can_place(&tetro, &Coords{col: WIDTH - 1, row: 5}), false);
    }
}
