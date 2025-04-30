mod imp;
pub mod gomoku;
pub mod player;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gomoku::GomokuColor;
use gtk::{glib, prelude::WidgetExt};
use player::PlayMode;

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

    pub(crate) fn set_play_mode(&self, play_mode: PlayMode) {
        self.imp().set_play_mode(play_mode);
    }

    pub(crate) fn set_ai_chess(&self, ai_chess: GomokuColor) {
        self.imp().set_ai_chess(ai_chess);
    }
}
