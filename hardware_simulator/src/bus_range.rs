#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BusRange {
    pub start: u16,
    pub end: u16,
}

impl BusRange {
    pub fn size(&self) -> u16 {
        self.end - self.start
    }
}
