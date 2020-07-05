use crate::updateable_view::{UpdatableView, Ctrl};
use crate::playfield::{Playfield, FieldTetrimino, Dir, HEIGHT};
use crate::figures::figures::{Shape};
use crate::view::{View, ShowArgs};
use crate::engine::engine::{LineStorage, Storable};

pub struct PlayfieldCtrl {
    view: UpdatableView,
    playfield: Playfield,
    no_ghost: bool,
    active_tetro: FieldTetrimino,
}

impl PlayfieldCtrl {
    pub fn move_active(self: &mut Self, dir: Dir) -> (bool, bool) {
        let move_result = self.playfield.move_tetro(&mut self.active_tetro, dir);
        let fall_space = self.playfield.has_fall_space(&mut self.active_tetro);
        if move_result {
            self.view.update();
        }

        (move_result, fall_space)
    }

    pub fn turn_active(self: &mut Self) -> (bool, bool) {
        let move_result = self.playfield.turn_tetro(&mut self.active_tetro);
        let fall_space = self.playfield.has_fall_space(&mut self.active_tetro);
        if move_result {
            self.view.update();
        };

        (move_result, fall_space)
    }

    pub fn place_active(self: &mut Self) {
        let _ = self.playfield.place(&self.active_tetro.tetro, self.active_tetro.coords);
        self.active_tetro.tetro.shape = Shape::NoShape;
        self.view.update();
    }

    pub fn new_active(self: &mut Self, tetro: FieldTetrimino) -> bool {
        self.active_tetro = tetro;
        self.view.update();
        self.playfield.can_place(&self.active_tetro.tetro, &self.active_tetro.coords)
    }

    pub fn active_shape(self: &Self) -> Shape {
        self.active_tetro.tetro.shape.clone()
    }

    pub fn remove_filled(self: &mut Self, lines: &mut LineStorage) {
        for l in lines {
            self.playfield.delete_row(l);
            self.view.update();
        }
    }

    pub fn find_filled(self: &mut Self, found: &mut dyn Storable) {
        for r in (0..HEIGHT).rev() {
            if self.playfield.row_filled(r) {
                found.store(r);
            }
        }
    }

    pub fn new(playfield: Playfield, no_ghost: bool) -> Self {
        PlayfieldCtrl{
            playfield: playfield,
            view: UpdatableView::default(),
            no_ghost: no_ghost,
            active_tetro: FieldTetrimino::default(),
        }
    }
}

impl Ctrl for PlayfieldCtrl {
    fn show(self: &mut Self, view: &impl View) {
        let ghost_tetro = if self.no_ghost {
            FieldTetrimino::default()
        } else {
            let mut ghost_tetro = self.active_tetro.clone();
            while self.playfield.move_tetro(&mut ghost_tetro, Dir::Down) {};
            ghost_tetro
        };

        self.view.show(view, &ShowArgs::PlayfieldArgs{
                                playfield: &self.playfield,
                                active_tetro: &self.active_tetro,
                                ghost_tetro: &ghost_tetro
                             });
    }
}