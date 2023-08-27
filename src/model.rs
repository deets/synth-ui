use std::time::Duration;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; // 0.8.0


const NOTE_COUNT:usize = 8;

#[derive(Copy, Clone)]
pub enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G
}

impl Distribution<Note> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Note {
        match rng.gen_range(0..=6) {
            0 => Note::A,
            1 => Note::B,
            2 => Note::C,
            3 => Note::D,
            4 => Note::E,
            5 => Note::F,
            6 => Note::G,
            _ => Note::G,
        }
    }
}

pub enum TransportState
{
    Stopped,
    Playing
}

pub struct Model
{
    pub bpm: f64,
    pub volume: f64,
    pub transport: TransportState,
    pub position: usize,
    pub notes: [Option<Note>; NOTE_COUNT],
    elapsed: Duration


}

impl Default for Model
{
    fn default() -> Self
    {
        Self {
            bpm: 120.0,
            volume: 5.0,
            transport: TransportState::Playing,
            position: 0,
            notes: [None; NOTE_COUNT],
            elapsed: Duration::from_secs(0),
        }
    }
}

impl Model {
    pub fn update(&mut self, elapsed: Duration)
    {
        match self.transport {
            TransportState::Playing => {
                self.elapsed += elapsed;
                self.position = ((self.elapsed.as_secs_f64() / 60.0 * self.bpm * 4.0).floor() as usize) % NOTE_COUNT;
            },
            _ => {}
        }
    }

    pub fn speedup(&mut self)
    {
        self.bpm = (self.bpm + 1.0).clamp(0.0, 300.0);
    }

    pub fn slowdown(&mut self)
    {
        self.bpm = (self.bpm - 1.0).clamp(0.0, 300.0);
    }


    pub fn pumpup(&mut self)
    {
        self.volume = (self.volume + 1.0).clamp(0.0, 11.0);
    }

    pub fn receive_complaint(&mut self)
    {
        self.volume = (self.volume - 1.0).clamp(0.0, 11.0);
    }

    pub fn toggle_transport(&mut self)
    {
        match self.transport {
            TransportState::Playing => {
                self.transport = TransportState::Stopped;
            }
            TransportState::Stopped => {
                self.transport = TransportState::Playing;
            }
        }
    }
}
