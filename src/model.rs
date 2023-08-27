use std::time::Duration;


const NOTE_COUNT:usize = 8;

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
    pub notes: [bool; NOTE_COUNT],
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
            notes: [false; NOTE_COUNT],
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
