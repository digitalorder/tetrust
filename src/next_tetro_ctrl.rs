use crate::updateable_view::UpdatableView;
use crate::view::{View, ShowArgs};
use crate::figures::figures::{Tetromino};

pub struct NextTetroCtrl {
    view: UpdatableView,
    next_tetro: Tetromino,
}

impl NextTetroCtrl {
    pub fn show(self: &mut Self, view: &impl View) {
        self.view.show(view, &ShowArgs::NextTetroArgs{tetro: self.next_tetro.clone()});
    }

    pub fn pop(self: &mut Self) -> Tetromino {
        let tetro = self.next_tetro.clone();
        self.next_tetro = Tetromino::new_random();
        self.view.update();
        tetro
    }

    pub fn new() -> Self {
        NextTetroCtrl{view: UpdatableView::default(), next_tetro: Tetromino::new_random()}
    }
}
