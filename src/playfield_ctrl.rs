use crate::updateable_view::{UpdatableView, Ctrl};
use crate::playfield::{Playfield, FieldTetrimino, Dir, HEIGHT};
use crate::figures::figures::{Shape};
use crate::view::{View, ShowArgs};

pub struct PlayfieldCtrl {
    view: UpdatableView,
    playfield: Playfield,
    no_ghost: bool,
    active_tetro: FieldTetrimino,
    filled_lines: LineStorage,
    animation_frame: u32,
    is_animating: bool
}

pub trait Storable {
    fn store(self: &mut Self, row: i8);
    fn elements(self: &Self) -> &[i8];
    fn reset(self: &mut Self);
}

#[derive(Clone)]
pub struct LineStorage {
    lines: [i8; 4],
    write_index: usize,
    read_index: usize,
}

impl Storable for LineStorage {
    fn store(self: &mut Self, row: i8) {
        assert_ne!(self.write_index, self.lines.len());
        self.lines[self.write_index] = row;
        self.write_index += 1;
    }

    fn reset(self: &mut Self) { self.write_index = 0; }
    fn elements(self: &Self) -> &[i8] {
        &self.lines[self.read_index..self.write_index]
    }
}

impl Default for LineStorage {
    fn default() -> LineStorage {
        LineStorage{lines: [0, 0, 0, 0], write_index: 0, read_index: 0}
    }
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

    pub fn remove_filled(self: &mut Self) -> usize {
        let removed_lines = self.filled_lines.elements().len();
        for l in self.filled_lines.elements() {
            self.playfield.delete_row(*l);
            self.view.update();
        }
        self.filled_lines.reset();
        removed_lines
    }

    fn find_filled(self: &mut Self) {
        for r in (0..HEIGHT).rev() {
            if self.playfield.row_filled(r) {
                self.filled_lines.store(r);
            }
        }
    }

    pub fn start_animation(self: &mut Self) {
        self.find_filled();
        self.animation_frame = 0;
        self.is_animating = self.filled_lines.elements().len() > 0;
    }

    pub fn animate(self: &mut Self) -> bool {
        self.view.update();
        self.is_animating
    }

    pub fn new(playfield: Playfield, no_ghost: bool) -> Self {
        PlayfieldCtrl{
            playfield: playfield,
            view: UpdatableView::default(),
            no_ghost: no_ghost,
            active_tetro: FieldTetrimino::default(),
            filled_lines: LineStorage::default(),
            animation_frame: 0,
            is_animating: false,
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

        let selected_lines = if self.is_animating && self.animation_frame % 30 > 15 {
            LineStorage::default()
        } else {
            self.filled_lines.clone()
        };
        self.animation_frame += 1;
        if self.animation_frame == 60 {
            self.is_animating = false;
        }

        self.view.show(view, &ShowArgs::PlayfieldArgs{
                                playfield: &self.playfield,
                                active_tetro: &self.active_tetro,
                                ghost_tetro: &ghost_tetro,
                                selected_lines: &selected_lines,
                             });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn line_storage_zero_on_creation() {
        let line_storage = LineStorage::default();
        assert_eq!(line_storage.elements().len(), 0);
    }

    #[test]
    fn line_storage_count_one() {
        let mut line_storage = LineStorage::default();
        line_storage.store(0);
        assert_eq!(line_storage.elements().len(), 1);
        assert_eq!(line_storage.elements()[0], 0);
    }

    #[test]
    fn line_storage_count_two() {
        let mut line_storage = LineStorage::default();
        line_storage.store(0);
        line_storage.store(1);
        assert_eq!(line_storage.elements().len(), 2);
        assert_eq!(line_storage.elements()[0], 0);
        assert_eq!(line_storage.elements()[1], 1);
    }

    #[test]
    fn line_storage_access_not_set() {
        let mut line_storage = LineStorage::default();
        line_storage.store(0);
        let result = panic::catch_unwind(|| {
            let _ = line_storage.elements()[2];
        });
        assert!(result.is_err());
    }
}
