use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};
use crate::engine::engine::{Mode};

pub struct EndgameCtrl {
    view: UpdatableView,
    game_over: bool,
}

impl Ctrl for EndgameCtrl {
    fn show(self: &mut Self, view: &mut impl View) {
        self.view.show(view, &ShowArgs::EndgameArgs{game_over: self.game_over});
    }
}

impl EndgameCtrl {
    pub fn update(self: &mut Self) {
        self.view.update();
    }

    pub fn new(mode: Mode) -> Self {
        EndgameCtrl{
            view: UpdatableView::new(false),
            game_over: mode == Mode::Marathon,
        }
    }
}
