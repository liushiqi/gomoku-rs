use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{
    CompositeTemplate,
    gio::prelude::ActionMapExtManual,
    glib::{
        self,
        variant::{StaticVariantType, ToVariant},
    },
};

use crate::game::{GomokuDrawing, gomoku::GomokuColor, player::PlayMode};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/cyou/liushiqi/Gomoku/gomoku.ui")]
pub struct GomokuWindow {
    #[template_child]
    pub new_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub undo_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub menu_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub popover_menu: TemplateChild<gtk::PopoverMenu>,
    #[template_child]
    pub main_menu: TemplateChild<gtk::gio::Menu>,
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

        let action_set_players = gtk::gio::ActionEntry::builder("set-players")
            .parameter_type(Some(&String::static_variant_type()))
            .state("SinglePlayer".to_variant())
            .activate(move |window: &super::Window, action, parameter| {
                // Get parameter
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<String>()
                    .expect("The value needs to be of type `String`.");

                let play_mode = match parameter.as_str() {
                    "SinglePlayer" => PlayMode::SinglePlayer,
                    "MultiplePlayer" => PlayMode::MultiplePlayer,
                    _ => unreachable!(),
                };

                // Set play mode
                window.imp().drawing.set_play_mode(play_mode);
                action.set_state(&parameter.to_variant());
            })
            .build();

        let action_set_ai_chess = gtk::gio::ActionEntry::builder("set-ai-chess")
            .parameter_type(Some(&String::static_variant_type()))
            .state("AIUseWhite".to_variant())
            .activate(move |window: &super::Window, action, parameter| {
                // Get parameter
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<String>()
                    .expect("The value needs to be of type `String`.");

                let ai_chess = match parameter.as_str() {
                    "AIUseWhite" => GomokuColor::White,
                    "AIUseBlack" => GomokuColor::Black,
                    _ => unreachable!(),
                };

                // Set play mode
                window.imp().drawing.set_ai_chess(ai_chess);
                action.set_state(&parameter.to_variant());
            })
            .build();

        self.drawing.set_play_mode(PlayMode::SinglePlayer);
        self.drawing.set_ai_chess(GomokuColor::White);
        self.obj()
            .add_action_entries([action_set_players, action_set_ai_chess]);
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
