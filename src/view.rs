use std::{time::{Duration, Instant}, cell::RefCell, rc::Rc};

use eframe::{egui::{self, Event, Key}, epaint::Rect};
use crate::model::Model;

pub trait View {
    fn feed(&mut self, event: Event, model: &mut Model) -> bool;
    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response;

    fn null_response(&self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_rect(Rect::from_center_size((0.0, 0.0).into(), (0.0, 0.0).into()), egui::Sense::click())
    }

    fn pressed(&self, candidate: Key, event: Event) -> bool {
        if let Event::Key{key, pressed, ..} = event {
            return key == candidate && pressed;
        }
        false
    }
}

pub struct MomentaryTimedView {
    view: Rc<RefCell<dyn View>>,
    key: Key,
    timeout: Duration,
    until: Instant,
}

impl MomentaryTimedView {
    pub fn new(view: Rc<RefCell<dyn View>>, key: Key, timeout: Duration) -> Self
    {
        Self {
            view, key, timeout, until: Instant::now()
        }
    }
    fn active(&self) -> bool {
        return self.until >= Instant::now()
    }
}


impl View for MomentaryTimedView
{
    fn feed(&mut self, event: Event, model: &mut Model) -> bool {
        if self.pressed(self.key, event.clone()) {
            self.until = Instant::now() + self.timeout;
            return true
        } else if self.active() {
            if self.view.borrow_mut().feed(event, model) {
                self.until = Instant::now() + self.timeout;
                return true;
            }
        }
        return false;
    }

    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response {
        if self.active() {
            self.view.borrow_mut().ui(ui, model)
        } else {
            // A bit abusive, we just don't care..
            self.null_response(ui)
        }

    }
}

pub struct ViewContainer {
    views: Vec<Rc<RefCell<dyn View>>>
}

impl ViewContainer {
    pub fn new(views: Vec<Rc<RefCell<dyn View>>>) -> Self
    {
        Self{ views }
    }
}


impl View for ViewContainer
{
    fn feed(&mut self, event: Event, model: &mut Model) -> bool {
        for view in &mut self.views {
            if view.borrow_mut().feed(event.clone(), model)
            {
                return true;
            }
        }
        false
    }

    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response {
        for view in &mut self.views {
            view.borrow_mut().ui(ui, model);
        }
        self.null_response(ui)
    }
}
