use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};

pub struct PlaytimeCtrl {
    frame_counter: u32,
    view: UpdatableView,
}

impl PlaytimeCtrl {
    pub fn update(self: &mut Self) {
        self.frame_counter += 1;
        if self.frame_counter % 7 == 0 {
            self.view.update();
        }
    }

    pub fn new() -> Self {
        PlaytimeCtrl {
            frame_counter: 0,
            view: UpdatableView::default(),
        }
    }
}

impl Ctrl for PlaytimeCtrl {
    fn show(self: &mut Self, view: &mut impl View) {
        let min = self.frame_counter / 60 / 60;
        let sec = self.frame_counter / 60 % 60;
        let csec = (self.frame_counter % 60) * 100 / 60;
        self.view.show(view, &ShowArgs::PlaytimeArgs{
            min, sec, csec
        });
    }
}
