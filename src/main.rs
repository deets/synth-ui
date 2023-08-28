#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![feature(let_chains)]

mod model;
mod view;
mod matrix;
mod root;
mod volume;

use eframe::egui;
use model::Model;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use view::View;
use root::Root;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 320.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Synth UI",
        options,
        Box::new(|_cc| Box::new(SynthUI::default())),
    )
}




struct SynthUI {
    model: Model,
    when: Instant,
    root: Rc<RefCell<dyn View>>,
}

impl Default for SynthUI {
    fn default() -> Self {
        Self {
            model: Model::default(),
            when: Instant::now(),
            root: Rc::new(RefCell::new(Root::default())),
        }
    }
}

impl SynthUI {

    fn dispatch_input_keys(&mut self, ctx: &egui::Context)
    {
        ctx.input(|i| {
            for event in &i.events {
                self.root.borrow_mut().feed(event.clone(), &mut self.model);
            }
        });
    }
}

impl eframe::App for SynthUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let elapsed = now - self.when;
        self.when = now;
        self.model.update(elapsed);
        self.dispatch_input_keys(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Synth UI");
            self.root.borrow_mut().ui(ui, &self.model);
        });
    }
}
