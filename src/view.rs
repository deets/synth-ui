use eframe::egui::{self, Event};
use crate::model::Model;

pub trait View {
    fn feed(&mut self, event: Event, model: &mut Model) -> bool;
    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response;
}
