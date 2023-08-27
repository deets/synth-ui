use eframe::{egui::{self, Event, Key}, epaint::{Color32, Stroke, vec2, Rounding, Rect}};

use crate::view::View;
use crate::model::Model;

const PADDING:f32 = 4.0;

pub struct MatrixView
{
    rect: Rect
}

impl Default for MatrixView
{
    fn default() -> Self {
        Self {
            rect: Rect::from_min_max((8.0, 60.0).into(), (312.0, 120.0).into())
        }
    }
}

impl View for MatrixView {
    fn feed(&mut self, event: Event, model: &mut Model) -> bool {
        for (i, value) in [Key::Num1, Key::Num2, Key::Num3, Key::Num4, Key::Num5, Key::Num6, Key::Num7].iter().enumerate() {
            if self.pressed(*value, event.clone()) {
                model.notes[i] = !model.notes[i];
                return true;
            }
        }
        // No consumption by us
        false
    }

    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response {
        let black = Color32::from_rgba_unmultiplied(0, 0, 0, 255);
        let white = Color32::from_rgba_unmultiplied(255, 255, 255, 255);
        let red = Color32::from_rgba_unmultiplied(255, 0, 0, 255);
        let rounding = Rounding::same(4.0);

        let width = self.rect.width();
        let note_count = model.notes.len() as f32;
        let pad_size = (width - (PADDING * (note_count - 1.0))) / note_count ;
        let response = ui.allocate_rect(self.rect, egui::Sense::click());
        if ui.is_rect_visible(self.rect) {
            ui.ctx().request_repaint();
            let painter = ui.painter_at(self.rect);
            let mut x = 0.0;
            for (i, note) in model.notes.iter().enumerate() {
                let rect = Rect::from_min_size(self.rect.left_top() + vec2(x, 2.0), vec2(pad_size, pad_size));
                let active_color = if i == model.position { red } else { white };
                if *note {
                    painter.rect_filled(rect, rounding, active_color);
                } else {
                    painter.rect(rect, rounding, black, Stroke::new(2.0, active_color));
                }
            x += pad_size + PADDING;
            }
        }
        response
    }
}
