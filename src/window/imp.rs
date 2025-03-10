use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{
    CompositeTemplate,
    glib,
};

use crate::drawing::GomokuDrawing;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/cyou/liushiqi/Gomoku/gomoku.ui")]
pub struct GomokuWindow {
    #[template_child]
    pub new_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub undo_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub title: TemplateChild<adw::WindowTitle>,
    #[template_child]
    pub drawing: TemplateChild<GomokuDrawing>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GomokuWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GomokuWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl GomokuWindow {
    #[template_callback]
    fn new_clicked(&self, _button: &gtk::Button) {
        self.drawing.new_game();
    }

    #[template_callback]
    fn undo_clicked(&self, _button: &gtk::Button) {
        self.drawing.undo();
    }
}

impl ObjectImpl for GomokuWindow {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for GomokuWindow {}

impl WindowImpl for GomokuWindow {
    fn close_request(&self) -> glib::Propagation {
        // Pass close request on to the parent
        self.parent_close_request()
    }
}

impl ApplicationWindowImpl for GomokuWindow {}

impl AdwApplicationWindowImpl for GomokuWindow {}
