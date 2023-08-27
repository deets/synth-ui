use eframe::egui::{Event, Key};

use crate::view::View;
use crate::matrix::MatrixView;
pub struct Root
{
    matrix: MatrixView
}

impl Default for Root {
    fn default() -> Self {
        Self { matrix: Default::default() }
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
}
