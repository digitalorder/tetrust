use crate::updateable_view::{UpdatableView, Ctrl};
use crate::playfield::{Playfield, FieldTetromino, Dir, HEIGHT};
use crate::figures::figures::{Shape};
use crate::view::{View, ShowArgs};

pub struct PlayfieldCtrl {
    view: UpdatableView,
    playfield: Playfield,
    no_ghost: bool,
    active_tetro: FieldTetromino,
}

impl PlayfieldCtrl {
    pub fn move_active(self: &mut Self, dir: Dir) -> bool {
        if self.playfield.move_tetro(&mut self.active_tetro, dir) {
            self.view.update();
            true
        } else {
            false
        }
    }

    pub fn turn_active(self: &mut Self) -> bool {
        if self.playfield.turn_tetro(&mut self.active_tetro) {
            self.view.update();
            true
        } else {
            false
        }
    }

    pub fn place_active(self: &mut Self) {
        let _ = self.playfield.place(&self.active_tetro.tetro, self.active_tetro.coords);
        self.active_tetro.tetro.shape = Shape::NoShape;
        self.view.update();
    }

    pub fn new_active(self: &mut Self, tetro: FieldTetromino) -> bool {
        self.active_tetro = tetro;
        self.view.update();
        self.playfield.can_place(&self.active_tetro.tetro, &self.active_tetro.coords)
    }

    pub fn remove_filled(self: &mut Self) -> u8 {
        let mut result = 0;

        for r in (0..HEIGHT).rev() {
            if self.playfield.row_filled(r) {
                result += 1;
                self.playfield.delete_row(r);
                self.view.update();
            }
        }

        result
    }

    pub fn new(playfield: Playfield, no_ghost: bool) -> Self {
        PlayfieldCtrl{
            playfield: playfield,
            view: UpdatableView::default(),
            no_ghost: no_ghost,
            active_tetro: FieldTetromino::default(),
        }
    }
}

impl Ctrl for PlayfieldCtrl {
    fn show(self: &mut Self, view: &impl View) {
        let ghost_tetro = if self.no_ghost {
            FieldTetromino::default()
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