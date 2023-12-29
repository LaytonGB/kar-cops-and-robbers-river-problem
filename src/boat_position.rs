#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoatPosition {
    Left,
    Right,
}

impl BoatPosition {
    pub fn switch(&self) -> Self {
        match self {
            BoatPosition::Left => BoatPosition::Right,
            BoatPosition::Right => BoatPosition::Left,
        }
    }
}
