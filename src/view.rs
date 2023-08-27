use std::{time::{Duration, Instant}, cell::RefCell, rc::Rc};

use eframe::{egui::{self, Event, Key}, epaint::Rect};
use crate::model::Model;

pub trait View {
    // Feed an event into the view. If it made use of the event,
    // the return should be true, and this signifies a stop of
    // this event's processing.
    fn feed(&mut self, event: Event, model: &mut Model) -> bool;
    // Render the view into the UI.
    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response;

    // We don't really use the response mechanism, as all we react to are keys. But
    // we needt to conform to protocol, so this creates a dummy response. Seems to work.
    fn null_response(&self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_rect(Rect::from_center_size((0.0, 0.0).into(), (0.0, 0.0).into()), egui::Sense::click())
    }

    // Utility function to check if a given key is pressed.
    fn pressed(&self, candidate: Key, event: Event) -> bool {
        if let Event::Key{key, pressed, ..} = event {
            return key == candidate && pressed;
        }
        false
    }

    // Utility function to check if a given key is involved
    fn used(&self, candidate: Key, event: Event) -> bool {
        if let Event::Key{key, ..} = event {
            return key == candidate
        }
        false
    }

    // Indicate if a view is active right now,
    // meaning it actually sinks events and is
    // drawing.
    fn active(&self) -> bool {
        // By default, all views are active.
        true
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

    fn active(&self) -> bool {
        return self.until >= Instant::now()
    }
}

pub struct MomentaryView {
    view: Rc<RefCell<dyn View>>,
    key: Key,
    active: bool,
}

impl MomentaryView {

    pub fn new(view: Rc<RefCell<dyn View>>, key: Key) -> Self
    {
        Self {
            view, key, active: false
        }
    }
}


impl View for MomentaryView
{
    fn feed(&mut self, event: Event, model: &mut Model) -> bool {
        if self.used(self.key, event.clone()) {
            if self.pressed(self.key, event.clone()) {
                self.active = true;
                return true
            } else {
                self.active = false;
                return true
            }
        } if self.active {
            return self.view.borrow_mut().feed(event, model);
        }
        return false;
    }

    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response {
        if self.active {
            self.view.borrow_mut().ui(ui, model)
        } else {
            self.null_response(ui)
        }

    }

    fn active(&self) -> bool {
        self.active
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

    fn active(&self) -> bool {
        self.views.iter().any(|view| { view.borrow().active() })
    }

}

pub struct MutexViewContainer {
    views: Vec<Rc<RefCell<dyn View>>>
}

impl MutexViewContainer {
    fn active_view(&self) -> Option<Rc<RefCell<dyn View>>>
    {
        for view in &self.views {
            if view.borrow().active() {
                return Some(view.clone())
            }
        }
        None
    }
}
impl View for MutexViewContainer {
    fn feed(&mut self, event: Event, model: &mut Model) -> bool {
        if let Some(view) =  self.active_view() {
            return view.borrow_mut().feed(event, model)
        }
        false
    }

    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response {
        if let Some(view) =  self.active_view() {
            return view.borrow_mut().ui(ui, model)
        }
        self.null_response(ui)
    }

    fn active(&self) -> bool {
        self.views.iter().any(|view| { view.borrow().active() })
    }
}
