use eframe::{egui::{Key, Sense}, epaint::{Rect, FontId, Color32, Rounding, Stroke}, emath::Align2};

use crate::view::View;

pub struct VolumeView {

}

impl Default for VolumeView
{
    fn default() -> Self {
        Self {  }
    }
}

impl View for VolumeView
{
    fn feed(&mut self, event: eframe::egui::Event, model: &mut crate::model::Model) -> bool {
        if self.pressed(Key::ArrowLeft, event.clone()) {
            model.receive_complaint();
            true
        } else if self.pressed(Key::ArrowRight, event.clone()) {
            model.pumpup();
            true
        } else {
            false
        }
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, model: &crate::model::Model) -> eframe::egui::Response {
        let black = Color32::from_rgba_unmultiplied(0, 0, 0, 255);
        let white = Color32::from_rgba_unmultiplied(255, 255, 255, 255);
        let red = Color32::from_rgba_unmultiplied(255, 0, 0, 255);
        let rounding = Rounding::same(4.0);

        let rect = Rect::from_min_max((0.0, 0.0).into(), (320.0, 160.0).into());
        let response = ui.allocate_rect(rect, Sense::click());
        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            painter.rect(
                Rect::from_center_size(rect.center(), (100.0, 100.0).into()), rounding, black, Stroke::new(2.0, white));
            painter.text(rect.center(), Align2::CENTER_CENTER, format!("{:}", model.volume), FontId::monospace(15.0), red);
        }
        response
    }
}
