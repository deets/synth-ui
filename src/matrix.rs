use eframe::{egui::{self, Painter}, epaint::{Color32, Stroke, pos2, vec2, Rounding, Rect}};

use crate::view::View;

pub struct MatrixView
{
}

impl View for MatrixView {
    fn feed(&mut self, key: egui::Event) {
    }
}

impl MatrixView {
    pub fn new() -> MatrixView {
        MatrixView {  }
    }
}

pub fn matrix_ui(ui: &mut egui::Ui) -> egui::Response {
    let width = ui.available_size_before_wrap().x;
    let pad_size = (width - (2.0 * 15.0)) / 16.0;
    let desired_size = egui::vec2(width, pad_size);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        ui.ctx().request_repaint();
        let painter = ui.painter_at(rect);
        let mut x = 0.0;
        for _ in 0..16 {
            let rect = Rect::from_min_size(rect.left_top() + vec2(x, 0.0), vec2(pad_size, pad_size));
            painter.rect_filled(rect, Rounding::same(4.0), Color32::from_rgba_unmultiplied(255, 255, 255, 255));
            x += pad_size + 2.0;
        }
    }
    response
}
