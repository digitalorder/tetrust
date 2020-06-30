use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};
use crate::figures::figures::{Tetromino, Shape};
use crate::playfield::{FieldTetromino, Coords, HEIGHT, WIDTH};

pub struct NextTetroCtrl {
    view: UpdatableView,
    next_tetro: Tetromino,
}

impl NextTetroCtrl {
    pub fn pop(self: &mut Self) -> FieldTetromino {
        let coords = match self.next_tetro.shape {
            Shape::LShape | Shape::TShape | Shape::JShape => Coords{row: HEIGHT, col: WIDTH / 2 - 2},
            _ => Coords{row: HEIGHT + 1, col: WIDTH / 2 - 2},
        };
        let tetro = FieldTetromino{
            coords: coords,
            tetro: self.next_tetro.clone(),
        };
        self.next_tetro = Tetromino::new_random();
        self.view.update();
        tetro
    }

    pub fn new() -> Self {
        NextTetroCtrl{view: UpdatableView::default(), next_tetro: Tetromino::new_random()}
    }
}

impl Ctrl for NextTetroCtrl {
    fn show(self: &mut Self, view: &impl View) {
        self.view.show(view, &ShowArgs::NextTetroArgs{tetro: self.next_tetro.clone()});
    }
}
