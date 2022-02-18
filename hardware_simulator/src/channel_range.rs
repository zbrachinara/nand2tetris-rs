#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ChannelRange {
    pub start: u16,
    pub end: u16,
}

impl ChannelRange {
    pub fn size(&self) -> u16 {
        self.end - self.start
    }
}
