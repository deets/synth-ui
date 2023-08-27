use eframe::egui::{Event, Key, Sense};
use eframe::emath::Align2;
use eframe::epaint::{Rect, Color32, FontId, vec2};

use crate::view::View;
use crate::matrix::MatrixView;
pub struct Root
{
    matrix: MatrixView,
    rect: Rect,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            matrix: Default::default(),
            rect: Rect::from_min_max((0.0, 0.0).into(), (320.0, 200.0).into())
        }
    }
}

impl Root {
    fn feed_internal(&mut self, event: Event, model: &mut crate::model::Model) -> bool {
        if let Event::Key{key, pressed, ..} = event {
            if pressed {
                match key {
                    Key::P => {
                        model.toggle_transport();
                        return true;
                    }
                    _ => { return false; }
                }
            }
        }
        false
    }
}

impl View for Root
{
    fn feed(&mut self, event: Event, model: &mut crate::model::Model) -> bool {
        if !self.matrix.feed(event.clone(), model) {
            return self.feed_internal(event, model)
        }
        true
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, model: &crate::model::Model) -> eframe::egui::Response {
        let response = ui.allocate_rect(self.rect, Sense::click());
        let painter = ui.painter_at(self.rect);
        let white = Color32::from_white_alpha(255);
        painter.text(self.rect.left_top() + vec2(8.0, 50.0), Align2::LEFT_CENTER, format!("{:}", model.bpm), FontId::monospace(15.0), white);
        self.matrix.ui(ui, model);
        response
    }

}
