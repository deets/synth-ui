const NOTE_COUNT:usize = 8;

pub struct Model
{
    pub notes: [bool; NOTE_COUNT]
}

impl Default for Model
{
    fn default() -> Self
    {
        Self {
            notes: [false; NOTE_COUNT]
        }
    }
}
