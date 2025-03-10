mod imp;
pub mod gomoku;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, prelude::WidgetExt};

glib::wrapper! {
    pub struct GomokuDrawing(ObjectSubclass<imp::GomokuDrawing>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GomokuDrawing {
    pub(crate) fn new_game(&self) {
        self.imp().new_game();
        self.queue_draw();
    }

    pub(crate) fn undo(&self) {
        self.imp().undo();
        self.queue_draw();
    }
}
