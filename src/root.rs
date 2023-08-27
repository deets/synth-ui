use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use eframe::egui::{Event, Key, Sense};
use eframe::emath::Align2;
use eframe::epaint::{Rect, Color32, FontId, vec2};

use crate::view::{View, ViewContainer, MomentaryTimedView};
use crate::matrix::MatrixView;
use crate::volume::VolumeView;
pub struct Root
{
    children: ViewContainer,
    rect: Rect,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            children: ViewContainer::new(vec![
                Rc::new(RefCell::new(MatrixView::default())),
                Rc::new(RefCell::new(MomentaryTimedView::new(
                    Rc::new(RefCell::new(VolumeView::default())),
                    Key::V, Duration::from_millis(1000))))
            ]),
            rect: Rect::from_min_max((0.0, 0.0).into(), (320.0, 200.0).into())
        }
    }
}

impl Root {
    fn feed_internal(&mut self, event: Event, model: &mut crate::model::Model) -> bool {
        if self.pressed(Key::P, event.clone()) {
            model.toggle_transport();
            return true;
        } else if self.pressed(Key::ArrowLeft, event.clone()) {
            model.slowdown();
            return true;
        } else if self.pressed(Key::ArrowRight, event.clone()) {
            model.speedup();
            return true;
        }
        false
    }
}

impl View for Root
{
    fn feed(&mut self, event: Event, model: &mut crate::model::Model) -> bool {
        if !self.children.feed(event.clone(), model) {
            return self.feed_internal(event, model)
        }
        true
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, model: &crate::model::Model) -> eframe::egui::Response {
        // First, render us
        let response = ui.allocate_rect(self.rect, Sense::click());
        let painter = ui.painter_at(self.rect);
        let white = Color32::from_white_alpha(255);
        painter.text(self.rect.left_top() + vec2(8.0, 50.0), Align2::LEFT_CENTER, format!("{:}", model.bpm), FontId::monospace(15.0), white);
        // then the child-views
        self.children.ui(ui, model);
        response
    }

}
