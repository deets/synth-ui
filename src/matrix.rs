use std::{cell::RefCell, rc::Rc};

use eframe::{egui::{self, Event, Key}, epaint::{Color32, Stroke, vec2, Rounding, Rect, pos2}};

use crate::{view::{View, MutexViewContainer, MomentaryView}, model::{Note, NOTE_COUNT}};
use crate::model::Model;
use rand::Rng;

const PADDING:f32 = 4.0;

fn note_color(note: &Option<Note>) -> Color32
{
    match note {
        Some(note) => {
            match note {
                Note::A => Color32::from_rgba_unmultiplied(0x9b, 0x5f, 0xe0, 0xff),
                Note::B => Color32::from_rgba_unmultiplied(0x16, 0xa4, 0xd8, 0xff),
                Note::C => Color32::from_rgba_unmultiplied(0x60, 0xdb, 0xe8, 0xff),
                Note::D => Color32::from_rgba_unmultiplied(0x8b, 0xd3, 0x46, 0xff),
                Note::E => Color32::from_rgba_unmultiplied(0xef, 0xdf, 0x48, 0xff),
                Note::F => Color32::from_rgba_unmultiplied(0xf9, 0xa5, 0x2c, 0xff),
                Note::G => Color32::from_rgba_unmultiplied(0xd6, 0x4e, 0x12, 0xff),
            }
        },
        None => Color32::from_rgba_unmultiplied(0, 0, 0, 255)
    }
}

fn selection_for_note(note: &Option<Note>) -> usize
{
    match note {
        Some(note) => {
            match note {
                Note::A => 0,
                Note::B => 1,
                Note::C => 2,
                Note::D => 3,
                Note::E => 4,
                Note::F => 5,
                Note::G => 6,
            }
        },
        None => 7
    }
}


fn note_for_selection(selection: usize) -> Option<Note>
{
    match selection {
        0 => { Some(Note::A)},
        1 => { Some(Note::B)},
        2 => { Some(Note::C)},
        3 => { Some(Note::D)},
        4 => { Some(Note::E)},
        5 => { Some(Note::F)},
        6 => { Some(Note::G)},
        _ => { None }
    }
}
struct NoteSelectionView
{
    rect: Rect,
    selection: usize,
    // Which note in the sequence
    number: usize,
}

impl View for NoteSelectionView
{
    fn activate(&mut self, model: &Model)
    {
        self.selection = selection_for_note(&model.notes[self.number]);
    }

    fn feed(&mut self, event: Event, model: &mut Model) -> bool {
        let mut res = false;
        if self.pressed(Key::ArrowLeft, event.clone()) {
            // usize needs guarding
            if self.selection > 0 {
                self.selection -= 1;
            }
            res = true;
        } else if self.pressed(Key::ArrowRight, event.clone()) {
            self.selection = (self.selection + 1).clamp(0, 7);
            res = true;
        }
        model.notes[self.number] = note_for_selection(self.selection);
        return res;
    }

    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response {
        let response = ui.allocate_rect(self.rect, egui::Sense::click());
        let rounding = Rounding::same(4.0);
        let white = Color32::from_gray(255);
        ui.ctx().request_repaint();
        let painter = ui.painter_at(self.rect);
        let pad_size = self.rect.width();
        painter.rect_filled(self.rect, rounding, Color32::from_gray(20));
        let mut selected = None;
        for (i, note) in [Some(Note::A), Some(Note::B), Some(Note::C), Some(Note::D), Some(Note::E), Some(Note::F), Some(Note::G), None].iter().enumerate() {
            let note_rect = Rect::from_min_size(self.rect.left_top() + vec2(0.0, (i as f32) * pad_size), (pad_size, pad_size).into());
            if self.selection == i {
                selected = Some((note_rect, note_color(note)));
            } else {
                painter.rect_filled(note_rect, rounding, note_color(note));
            }
        }
        if let Some((note_rect, color)) = selected {
            painter.rect(note_rect, rounding, color, Stroke::new(2.0, white));
        }

        response
    }
}

pub struct MatrixView
{
    rect: Rect,
    note_views: MutexViewContainer,
}

impl Default for MatrixView
{
    fn default() -> Self {
        let rect = Rect::from_min_max((8.0, 60.0).into(), (312.0, 120.0).into());
        let mut views: std::vec::Vec<Rc<RefCell<dyn View>>> = vec![];
        let pad_size = (rect.width() - (PADDING * (NOTE_COUNT as f32 - 1.0))) / NOTE_COUNT as f32 ;
        for (i, value) in [Key::Num1, Key::Num2, Key::Num3, Key::Num4, Key::Num5, Key::Num6, Key::Num7, Key::Num8].iter().enumerate() {
            let note_rect = Rect::from_min_size(
                pos2(rect.left(), PADDING) + vec2(i as f32 * (pad_size + PADDING), 0.0),
                vec2(pad_size, (pad_size + PADDING) * 8.0 as f32 - pad_size + PADDING)); // The 8 is 7 notes + no note
            views.push(Rc::new(RefCell::new(
                MomentaryView::new(
                    Rc::new(RefCell::new(NoteSelectionView { rect: note_rect, selection: 0, number: i })),
                    *value,
                )
            )));
        }
        Self {
            rect,
            note_views: MutexViewContainer{ views }
        }
    }
}


impl View for MatrixView {
    fn feed(&mut self, event: Event, model: &mut Model) -> bool {
        self.note_views.feed(event, model)
    }

    fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> egui::Response {
        let black = Color32::from_rgba_unmultiplied(0, 0, 0, 255);
        let red = Color32::from_rgba_unmultiplied(255, 0, 0, 255);
        let rounding = Rounding::same(4.0);

        let width = self.rect.width();
        let note_count = model.notes.len() as f32;
        let pad_size = (width - (PADDING * (note_count - 1.0))) / note_count ;

        if ui.is_rect_visible(self.rect) {
            ui.ctx().request_repaint();
            let painter = ui.painter_at(self.rect);
            let mut x = 0.0;
            for (i, note) in model.notes.iter().enumerate() {
                let rect = Rect::from_min_size(self.rect.left_top() + vec2(x, 2.0), vec2(pad_size, pad_size));
                let active_color = if i == model.position { red } else { note_color(&note) };
                if let Some(note) = *note {
                    painter.rect_filled(rect, rounding, note_color(&Some(note)));
                } else {
                    painter.rect(rect, rounding, black, Stroke::new(2.0, active_color));
                }
            x += pad_size + PADDING;
            }
        }
        self.note_views.ui(ui, model)
    }
}
