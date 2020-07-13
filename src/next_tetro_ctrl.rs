use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};
use crate::figures::figures::{Tetrimino, Shape};
use crate::playfield::{FieldTetrimino, Coords, HEIGHT, WIDTH};
use rand::{thread_rng};
use rand::seq::SliceRandom;

pub const PREVIEW_SIZE: usize = 4;
const DRAW_SIZE: usize = 7;

pub struct NextTetroCtrl {
    view: UpdatableView,
    bag: [Shape; DRAW_SIZE * 2],
    bag_index: usize,
    pushed_flag: bool,
}

pub struct AlreadyPushed;

impl NextTetroCtrl {

    fn draw_next(self: &mut Self) {
        if self.bag_index < DRAW_SIZE - 1 {
            self.bag_index += 1;
        } else {
            let (left, right) = self.bag.split_at_mut(DRAW_SIZE);
            left.clone_from_slice(right);
            right.clone_from_slice(&NextTetroCtrl::shuffle_bag());
            self.bag_index = 0;
        };
        self.pushed_flag = false;
    }

    pub fn pop(self: &mut Self) -> FieldTetrimino {
        let shape = self.get_current_shape()[0].clone();
        self.draw_next();
        let coords = match shape {
            Shape::LShape | Shape::TShape | Shape::JShape => Coords{row: HEIGHT, col: WIDTH / 2 - 2},
            _ => Coords{row: HEIGHT + 1, col: WIDTH / 2 - 2},
        };
        let tetro = FieldTetrimino{
            coords: coords,
            tetro: Tetrimino::new(shape),
        };
        self.view.update();
        tetro
    }

    pub fn swap(self: &mut Self, shape: Shape) -> Result<(FieldTetrimino), AlreadyPushed> {
        if self.pushed_flag {
            return Err(AlreadyPushed{});
        }

        let popped = self.pop();
        if self.bag_index > 0 {
            self.bag_index -= 1;
        } else {
            self.bag_index = DRAW_SIZE - 1;
        }
        self.bag[self.bag_index] = shape;
        self.pushed_flag = true;
        Ok(popped)
    }

    fn shuffle_bag() -> [Shape; DRAW_SIZE] {
        let mut rnd = thread_rng();
        let mut bag = [Shape::OShape, Shape::IShape,
                       Shape::TShape, Shape::JShape,
                       Shape::LShape, Shape::SShape,
                       Shape::ZShape,];
        bag.shuffle(&mut rnd);
        bag
    }

    pub fn new() -> Self {
        /* do two shuffles and put them immediately inside bag */
        let mut bag: [Shape; DRAW_SIZE * 2] = Default::default();
        bag[0..DRAW_SIZE].clone_from_slice(&NextTetroCtrl::shuffle_bag());
        bag[DRAW_SIZE..].clone_from_slice(&NextTetroCtrl::shuffle_bag());
        NextTetroCtrl{
            view: UpdatableView::default(),
            bag: bag,
            bag_index: 0,
            pushed_flag: false
        }
    }
}

impl Ctrl for NextTetroCtrl {
    fn show(self: &mut Self, view: &impl View) {
        self.view.show(view, &ShowArgs::NextTetroArgs{
            next: &self.bag[self.bag_index..self.bag_index + PREVIEW_SIZE]
        });
    }
}
