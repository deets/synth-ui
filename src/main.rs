#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![feature(let_chains)]

mod view;
mod matrix;

use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

use view::View;
use matrix::MatrixView;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Synth UI",
        options,
        Box::new(|_cc| Box::new(SynthUI::default())),
    )
}




struct SynthUI {
    bpm: f32,
    position: f32,
    root: Rc<RefCell<dyn View>>,
}

impl Default for SynthUI {
    fn default() -> Self {
        Self {
            bpm: 120.0,
            position: 0.0,
            root: Rc::new(RefCell::new(MatrixView::new())),
        }
    }
}

impl SynthUI {

    fn dispatch_input_keys(&mut self, ctx: &egui::Context)
    {
        ctx.input(|i| {
            for event in &i.events {
                self.root.borrow_mut().feed(event.clone());
            }
        });
    }
}

impl eframe::App for SynthUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.dispatch_input_keys(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Synth UI");
            matrix::matrix_ui(ui);
        });
    }
}
