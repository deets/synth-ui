use eframe::egui::Event;
use crate::model::Model;

pub trait View {
    fn feed(&mut self, event: Event, model: &mut Model);
}
