use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};

pub struct StaticCtrl {
    view: UpdatableView,
    next_queue_size: i8
}

impl Ctrl for StaticCtrl {
    fn show(self: &mut Self, view: &mut impl View) {
        self.view.show(view, &ShowArgs::StaticArgs{next_queue_size: self.next_queue_size});
    }
}

impl StaticCtrl {
    pub fn new(next_queue_size: usize) -> Self {
        StaticCtrl{
            view: UpdatableView::default(),
            next_queue_size: next_queue_size as i8,
        }
    }
}

