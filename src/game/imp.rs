use std::{cell::RefCell, ops::Deref, sync::Arc};

use adw::prelude::{AlertDialogExt, AlertDialogExtManual};
use gtk::{
    CompositeTemplate,
    glib::{self, object::IsA},
    prelude::{GestureSingleExt, SnapshotExt, WidgetExt},
    subclass::prelude::*,
};

use crate::ai::next_chess;

use super::{
    gomoku::{GomokuChess, GomokuColor, GomokuData},
    player::PlayMode,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/cyou/liushiqi/Gomoku/drawing.ui")]
pub struct GomokuDrawing {
    pub data: Arc<RefCell<GomokuData>>,
    pub board_width: RefCell<f32>,
    pub finished: RefCell<bool>,
    pub play_mode: RefCell<PlayMode>,
    pub ai_chess: RefCell<GomokuColor>,
}

#[glib::object_subclass]
impl ObjectSubclass for GomokuDrawing {
    const NAME: &'static str = "GomokuDrawing";
    type Type = super::GomokuDrawing;
    type ParentType = gtk::Widget;
}

impl GomokuDrawing {
    pub(crate) fn new_game(&self) {
        self.finished.replace(false);
        self.data.replace(GomokuData::default());
    }

    pub(crate) fn undo(&self) {
        let mut data = self.data.borrow_mut();
        self.finished.replace(false);
        data.undo();
    }

    pub(crate) fn set_play_mode(&self, play_mode: PlayMode) {
        self.play_mode.replace(play_mode);
    }

    pub(crate) fn set_ai_chess(&self, ai_chess: GomokuColor) {
        self.ai_chess.replace(ai_chess);
    }

    async fn winner<W: IsA<gtk::Widget>>(&self, winner: GomokuColor, window: W) {
        let dialog = adw::AlertDialog::builder()
            .heading("Finished!")
            .body(format!("{winner} Chess Wins"))
            .default_response("ok")
            .close_response("new")
            .build();
        dialog.add_responses(&[("ok", "Ok"), ("new", "New Game")]);
        dialog.set_response_appearance("new", adw::ResponseAppearance::Destructive);

        let option = dialog.choose_future(&window).await;
        match option.as_str() {
            "ok" => {}
            "new" => {
                self.new_game();
            }
            _ => {}
        }
    }

    async fn clicked<W: IsA<gtk::Widget>>(&self, x: f64, y: f64, window: W) {
        let data = self.data.borrow();
        if *self.play_mode.borrow().deref() == PlayMode::SinglePlayer
            && data.current == self.ai_chess.borrow().deref().clone()
        {
            return;
        }
        drop(data);

        let mut data = self.data.borrow_mut();

        if !self.finished.borrow().deref().clone() {
            let board_width = self.board_width.borrow().deref().clone();
            let size = data.size;
            let width = size as f32 * board_width;
            let horizontal_padding = (self.obj().width() as f32 - width) / 2.0;
            let vertical_padding = (self.obj().height() as f32 - width) / 2.0;
            let x = x as f32 - horizontal_padding;
            let y = y as f32 - vertical_padding;
            let x = (x / board_width).round() as u32;
            let y = (y / board_width).round() as u32;
            if x <= size && y <= size {
                data.place(x, y);
                drop(data);
                self.obj().queue_draw();
            }
        }

        let mut data = self.data.borrow_mut();

        let winner = data.has_winner();

        if self.play_mode.borrow().deref().clone() == PlayMode::SinglePlayer
            && !self.finished.borrow().deref().clone()
            && winner.is_none()
        {
            let ai_chess = self.ai_chess.borrow().deref().clone();
            let current = data.current;
            if current == ai_chess {
                let next_chess = next_chess(&data).await;
                data.place(next_chess.x, next_chess.y);
            }
        }

        let winner = data.has_winner();

        if let Some(winner) = winner {
            self.obj().queue_draw();
            self.winner(winner, window).await;
        }
    }
}

impl ObjectImpl for GomokuDrawing {
    fn constructed(&self) {
        self.parent_constructed();

        let win = self.obj();
        let w = win.clone();

        self.board_width.replace(30.0);

        self.new_game();

        let gesture = gtk::GestureClick::builder().button(0).build();
        gesture.connect_released(move |gesture, _, x, y| match gesture.current_button() {
            1 => {
                let cb = async |x: f64, y: f64, w: super::GomokuDrawing| {
                    w.imp().clicked(x, y, w.clone()).await
                };
                gtk::glib::MainContext::default().spawn_local(cb(x, y, w.clone()));
            }
            _ => {}
        });
        self.obj().add_controller(gesture);
    }
}

impl WidgetImpl for GomokuDrawing {
    // We override the snapshot virtual function to draw custom graphics
    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        // Call the parent snapshot function to draw the background
        self.parent_snapshot(snapshot);

        // Draw custom graphics here
        let window_width = self.obj().width() as f32;
        let window_height = self.obj().height() as f32;

        let board_width = self.board_width.borrow().deref().clone();

        let data = self.data.borrow();
        let size = data.size;

        let color = self.obj().color();
        let length = size as f32 * board_width;
        let horizontal_padding = (window_width - length) / 2.0;
        let vertical_padding = (window_height - length) / 2.0;

        snapshot.translate(&gtk::graphene::Point::new(
            horizontal_padding,
            vertical_padding,
        ));

        // draw a gomoku grid
        for i in 0..=size {
            let height = if i == 0 || i == size { 2.0 } else { 1.0 };
            snapshot.append_color(
                &color,
                &gtk::graphene::Rect::new(
                    -1.0,
                    i as f32 * board_width - height / 2.0,
                    length + 2.0,
                    height,
                ),
            );
        }

        for i in 0..=size {
            let width = if i == 0 || i == size { 2.0 } else { 1.0 };
            snapshot.append_color(
                &color,
                &gtk::graphene::Rect::new(
                    i as f32 * board_width - width / 2.0,
                    -1.0,
                    width,
                    length + 2.0,
                ),
            );
        }

        for i in 0..9 {
            let x = match i % 3 {
                0 => board_width * 3.0,
                1 => board_width * 9.0,
                _ => board_width * 15.0,
            };
            let y = match i / 3 {
                0 => board_width * 3.0,
                1 => board_width * 9.0,
                _ => board_width * 15.0,
            };

            let circle = gtk::gsk::PathBuilder::new();
            circle.add_circle(&gtk::graphene::Point::new(x, y), 3.0);
            let circle = circle.to_path();
            snapshot.append_fill(&circle, gtk::gsk::FillRule::Winding, &color);
        }

        // draw gomoku grid
        for GomokuChess {
            x,
            y,
            color: chess_color,
        } in data.iter()
        {
            let x = *x as f32 * board_width;
            let y = *y as f32 * board_width;
            let white = gtk::gdk::RGBA::new(1.0, 1.0, 1.0, 1.0);
            let black = gtk::gdk::RGBA::new(0.0, 0.0, 0.0, 1.0);
            let chess_color = match chess_color {
                GomokuColor::White => &white,
                GomokuColor::Black => &black,
            };
            let circle = gtk::gsk::PathBuilder::new();
            circle.add_circle(&gtk::graphene::Point::new(x, y), board_width / 2.0 - 3.0);
            let circle = circle.to_path();
            snapshot.append_stroke(&circle, &gtk::gsk::Stroke::new(3.0), &color);
            snapshot.append_fill(&circle, gtk::gsk::FillRule::Winding, &chess_color);
        }
    }
}
