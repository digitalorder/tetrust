use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};
use crate::figures::figures::{Tetromino, Shape};
use crate::playfield::{FieldTetromino, Coords, HEIGHT, WIDTH};
use rand::{thread_rng};
use rand::seq::SliceRandom;

pub struct NextTetroCtrl {
    view: UpdatableView,
    bag: [Shape; 7],
    bag_index: usize,
    pushed_flag: bool,
}

pub struct AlreadyPushed;

impl NextTetroCtrl {
    fn get_current_shape(self: &Self) -> Shape {
        self.bag[self.bag_index].clone()
    }

    fn draw_next(self: &mut Self) {
        if self.bag_index < self.bag.len() - 1 {
            self.bag_index += 1;
        } else {
            self.bag = NextTetroCtrl::shuffle_bag();
            self.bag_index = 0;
        };
        self.pushed_flag = false;
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

    pub fn swap(self: &mut Self, shape: Shape) -> Result<(FieldTetromino), AlreadyPushed> {
        if self.pushed_flag {
            return Err(AlreadyPushed{});
        }

        let popped = self.pop();
        if self.bag_index > 0 {
            self.bag_index -= 1;
        } else {
            self.bag_index = self.bag.len() - 1;
        }
        self.bag[self.bag_index] = shape;
        self.pushed_flag = true;
        Ok(popped)
    }

    fn shuffle_bag() -> [Shape; 7] {
        let mut rnd = thread_rng();
        let mut bag = [Shape::OShape, Shape::IShape,
                       Shape::TShape, Shape::JShape,
                       Shape::LShape, Shape::SShape,
                       Shape::ZShape,];
        bag.shuffle(&mut rnd);
        bag
    }

    pub fn new() -> Self {
        NextTetroCtrl{
            view: UpdatableView::default(),
            bag: NextTetroCtrl::shuffle_bag(),
            bag_index: 0,
            pushed_flag: false
        }
    }
}

impl Ctrl for NextTetroCtrl {
    fn show(self: &mut Self, view: &impl View) {
        self.view.show(view, &ShowArgs::NextTetroArgs{next: self.get_current_shape()});
    }
}
