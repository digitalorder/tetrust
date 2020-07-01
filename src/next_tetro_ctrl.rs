use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};
use crate::figures::figures::{Tetromino, Shape};
use crate::playfield::{FieldTetromino, Coords, HEIGHT, WIDTH};
use rand::{thread_rng};
use rand::seq::SliceRandom;

pub struct NextTetroCtrl {
    view: UpdatableView,
    bag: [u8; 7],
    bag_index: usize,
}

impl NextTetroCtrl {
    fn get_current_shape(self: &Self) -> Shape {
        let shape_num = self.bag[self.bag_index];
        match shape_num {
            1 => Shape::OShape,
            2 => Shape::IShape,
            3 => Shape::TShape,
            4 => Shape::JShape,
            5 => Shape::LShape,
            6 => Shape::SShape,
            _ => Shape::ZShape,
        }
    }

    fn draw_next(self: &mut Self) {
        if self.bag_index < self.bag.len() - 1 {
            self.bag_index += 1;
        } else {
            self.bag = NextTetroCtrl::shuffle_bag();
            self.bag_index = 0;
        };
    }

    pub fn pop(self: &mut Self) -> FieldTetromino {
        let shape = self.get_current_shape();
        self.draw_next();
        let coords = match shape {
            Shape::LShape | Shape::TShape | Shape::JShape => Coords{row: HEIGHT, col: WIDTH / 2 - 2},
            _ => Coords{row: HEIGHT + 1, col: WIDTH / 2 - 2},
        };
        let tetro = FieldTetromino{
            coords: coords,
            tetro: Tetromino::new(shape),
        };
        self.view.update();
        tetro
    }

    fn shuffle_bag() -> [u8; 7] {
        let mut rnd = thread_rng();
        let mut bag = [1, 2, 3, 4, 5, 6, 7];
        bag.shuffle(&mut rnd);
        bag
    }

    pub fn new() -> Self {
        NextTetroCtrl{view: UpdatableView::default(), bag: NextTetroCtrl::shuffle_bag(), bag_index: 0}
    }
}

impl Ctrl for NextTetroCtrl {
    fn show(self: &mut Self, view: &impl View) {
        let tetro = Tetromino::new(self.get_current_shape());
        self.view.show(view, &ShowArgs::NextTetroArgs{tetro: tetro});
    }
}
