use crate::updateable_view::UpdatableView;
use crate::view::{View, ShowArgs};

pub struct StaticCtrl {
    view: UpdatableView,
}

impl StaticCtrl {
    pub fn show(self: &mut Self, view: &impl View) {
        self.view.show(view, &ShowArgs::StaticArgs{});
    }
}

impl Default for StaticCtrl {
    fn default() -> Self {StaticCtrl{view: UpdatableView::default()}}
}

