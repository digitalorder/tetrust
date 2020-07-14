use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};

pub struct StaticCtrl {
    view: UpdatableView,
}

impl Ctrl for StaticCtrl {
    fn show(self: &mut Self, view: &mut impl View) {
        self.view.show(view, &ShowArgs::StaticArgs{});
    }
}

impl Default for StaticCtrl {
    fn default() -> Self {StaticCtrl{view: UpdatableView::default()}}
}

