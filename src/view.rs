use eframe::egui::Event;

pub trait View {
    fn feed(&mut self, key: Event);
}
