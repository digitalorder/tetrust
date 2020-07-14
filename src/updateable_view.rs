use crate::view::{View, ShowArgs};

pub struct UpdatableView {
    updated: bool,
}

impl UpdatableView {
    pub fn update(self: &mut Self) {
        self.updated = true;
    }

    pub fn show(self: &mut Self, view: &mut impl View, args: &ShowArgs) {
        if self.updated {
            view.show_subview(args);
        }
        self.updated = false;
    }
}

impl Default for UpdatableView {
    fn default() -> Self {UpdatableView{updated: true}}
}

pub trait Ctrl {
    fn show(self: &mut Self, view: &mut impl View);
}
