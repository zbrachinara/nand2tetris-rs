use std::ops::Range;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ChannelRange {
    pub start: u16,
    pub end: u16,
}

impl ChannelRange {
    pub fn size(&self) -> u16 {
        self.end - self.start
    }
    pub fn as_range(&self) -> Range<usize> {
        self.clone().into()
    }
}

impl From<ChannelRange> for Range<usize> {
    fn from(r: ChannelRange) -> Self {
        (r.start as usize)..(r.end as usize + 1)
    }
}

#[cfg(test)]
mod test {
    use crate::channel_range::ChannelRange;

    #[test]
    fn to_std_range() {
        assert_eq!(ChannelRange {
            start: 0, end: 10
        }.as_range(), 0..11)
    }
}