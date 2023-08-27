use eframe::{egui::{self, Painter, Event, Key}, epaint::{Color32, Stroke, pos2, vec2, Rounding, Rect}};

use crate::view::View;
use crate::model::Model;

const PADDING:f32 = 4.0;

pub struct MatrixView
{
}

impl Default for MatrixView
{
    fn default() -> Self {
        Self {  }
    }
}

impl View for MatrixView {
    fn feed(&mut self, event: Event, model: &mut Model) -> bool {
        if let Event::Key{key, pressed, ..} = event {
            if pressed {
                for (i, value) in [Key::Num1, Key::Num2, Key::Num3, Key::Num4, Key::Num5, Key::Num6, Key::Num7].iter().enumerate() {
                    if key == *value {
                        model.notes[i] = !model.notes[i];
                        return true;
                    }
                }
            }
        }
        // No consumption by us
        false
    }
}


pub fn matrix_ui(ui: &mut egui::Ui, model: &Model) -> egui::Response {
    let black = Color32::from_rgba_unmultiplied(0, 0, 0, 255);
    let white = Color32::from_rgba_unmultiplied(255, 255, 255, 255);
    let red = Color32::from_rgba_unmultiplied(255, 0, 0, 255);
    let rounding = Rounding::same(4.0);

    let width = ui.available_size_before_wrap().x;
    let note_count = model.notes.len() as f32;
    let pad_size = (width - (PADDING * (note_count - 1.0))) / note_count ;
    let desired_size = egui::vec2(width, pad_size);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        ui.ctx().request_repaint();
        let painter = ui.painter_at(rect);
        let mut x = 0.0;
        for (i, note) in model.notes.iter().enumerate() {
            let rect = Rect::from_min_size(rect.left_top() + vec2(x, 0.0), vec2(pad_size, pad_size));
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
