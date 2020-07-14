use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};
use crate::figures::figures::{Tetrimino, Shape};
use crate::playfield::{FieldTetrimino, Coords, HEIGHT, WIDTH};
use rand::{thread_rng};
use rand::seq::SliceRandom;

pub const PREVIEW_SIZE: usize = 4;
const DRAW_SIZE: usize = 7;
const BAG_SIZE: usize = DRAW_SIZE * 2;

pub struct NextTetroCtrl {
    view: UpdatableView,
    bag: [Shape; BAG_SIZE],
    bag_index: usize,
    pushed_flag: bool,
}

pub struct AlreadyPushed;

impl NextTetroCtrl {
    /* Consume next item in upcoming queue and fill in empty spaces if they occur */
    fn draw_next(self: &mut Self) -> Shape {
        let result = self.bag[self.bag_index].clone();
        if self.bag_index < DRAW_SIZE - 1 {
            self.bag_index += 1;
        } else {
            /* used up current draw, time to generate a new one */
            let (left, right) = self.bag.split_at_mut(DRAW_SIZE);
            left.clone_from_slice(right);
            right.clone_from_slice(&NextTetroCtrl::shuffle_bag());
            self.bag_index = 0;
        };
        self.pushed_flag = false;
        result
    }

    /* Consume next item from upcoming queue and make a proper Tetrimino out of it */
    pub fn pop(self: &mut Self) -> FieldTetrimino {
        self.view.update();
        let shape = self.draw_next();
        FieldTetrimino{
            coords: match shape {
                Shape::LShape | Shape::TShape | Shape::JShape => Coords{row: HEIGHT, col: WIDTH / 2 - 2},
                _ => Coords{row: HEIGHT + 1, col: WIDTH / 2 - 2},
            },
            tetro: Tetrimino::new(shape),
        }
    }

    /* Replace given shape with whatever is on top of upcoming queue */
    pub fn swap(self: &mut Self, shape: Shape) -> Result<(FieldTetrimino), AlreadyPushed> {
        if self.pushed_flag {
            return Err(AlreadyPushed{});
        }

        let popped = self.pop();
        self.push(shape);
        Ok(popped)
    }

    /* Make given shape next in draw */
    fn push(self: &mut Self, shape: Shape) {
        if self.bag_index > 0 {
            self.bag_index -= 1;
        } else {
            self.bag_index = DRAW_SIZE - 1;
        }
        self.bag[self.bag_index] = shape;
        self.pushed_flag = true;
    }

    /* Produce new rearrangement of 7-tetro bag */
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
        let mut bag: [Shape; BAG_SIZE] = Default::default();
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
    fn show(self: &mut Self, view: &mut impl View) {
        self.view.show(view, &ShowArgs::NextTetroArgs{
            next: &self.bag[self.bag_index..self.bag_index + PREVIEW_SIZE]
        });
    }
}
