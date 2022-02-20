use std::ops::RangeInclusive;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ChannelRange {
    pub start: u16,
    pub end: u16,
}

impl ChannelRange {
    pub fn size(&self) -> u16 {
        self.end - self.start + 1 // TODO: make this operation checked
    }
    pub fn as_range(&self) -> RangeInclusive<usize> {
        self.clone().into()
    }
}

impl From<ChannelRange> for RangeInclusive<usize> {
    fn from(r: ChannelRange) -> Self {
        (r.start as usize)..=(r.end as usize)
    }
}

#[cfg(test)]
mod test {
    use crate::channel_range::ChannelRange;

    #[test]
    fn to_std_range() {
        assert_eq!(ChannelRange { start: 0, end: 10 }.as_range(), 0..=10)
    }
}
