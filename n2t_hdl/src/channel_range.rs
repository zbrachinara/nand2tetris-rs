use ::std::ops::RangeInclusive;

use funty::Integral;

/// The range of a sub-bus of a non-internal bus (i.e., specified as IN or OUT)
///
/// Inclusive on `start` and `end`
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct ChannelRange {
    pub(crate) start: u16,
    pub(crate) end: u16,
}

impl ChannelRange {
    /// Creates a new `ChannelRange` with the specified start and end
    ///
    /// # Panics
    ///
    /// Panics if `start` > `end`
    pub fn new(start: u16, end: u16) -> Self {
        assert!(start <= end, "Channel range start can't be past end");
        Self { start, end }
    }

    /// Returns the width of the sub-bus
    pub fn size(&self) -> usize {
        // Can't underflow because `self.start` <= `self.end`
        usize::from(self.end - self.start + 1)
    }

    /// Returns a range from `start` to `end`
    pub fn as_range(&self) -> RangeInclusive<usize> {
        (*self).into()
    }

    /// Returns the value of `start`
    pub fn start(&self) -> u16 {
        self.start
    }

    /// Returns the value of `end`
    pub fn end(&self) -> u16 {
        self.end
    }

    /// Sets the value of `start` to the specified value
    ///
    /// # Panics
    ///
    /// Panics if `start` > `end`
    pub fn set_start(&mut self, start: u16) {
        assert!(start <= self.end, "Channel range start can't be past end");
        self.start = start;
    }

    /// Sets the value of `end` to the specified value
    ///
    /// # Panics
    ///
    /// Panics if `start` > `end`
    pub fn set_end(&mut self, end: u16) {
        assert!(self.start <= end, "Channel range start can't be past end");
        self.end = end;
    }
}

impl From<ChannelRange> for RangeInclusive<usize> {
    fn from(range: ChannelRange) -> Self {
        usize::from(range.start)..=usize::from(range.end)
    }
}

impl<T: Integral> From<RangeInclusive<T>> for ChannelRange {
    fn from(r: RangeInclusive<T>) -> Self {
        match <T as TryInto<u16>>::try_into(*r.start()).and_then(|start| {
            <T as TryInto<u16>>::try_into(*r.end()).map(|end| ChannelRange { start, end })
        }) {
            Ok(x) => x,
            Err(_) => panic!("could not resolve the given range to a u16 range"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ChannelRange;

    #[test]
    fn new_range() {
        assert_eq!(ChannelRange::new(0, 5), ChannelRange { start: 0, end: 5 });
        assert_eq!(ChannelRange::new(5, 5), ChannelRange { start: 5, end: 5 });
    }

    #[test]
    #[should_panic]
    fn new_invalid_range() {
        ChannelRange::new(5, 0);
    }

    #[test]
    fn range_size() {
        assert_eq!(ChannelRange::new(0, 5).size(), 6);
        assert_eq!(ChannelRange::new(5, 5).size(), 1);
    }

    #[test]
    fn access_range_bounds() {
        let mut range = ChannelRange::new(0, 5);
        assert_eq!(range.start(), 0);
        assert_eq!(range.end(), 5);
        range.set_start(4);
        range.set_end(6);
        assert_eq!(range.start, 4);
        assert_eq!(range.start(), 4);
        assert_eq!(range.end, 6);
        assert_eq!(range.end(), 6);
    }

    #[test]
    #[should_panic]
    fn set_invalid_range_start() {
        let mut range = ChannelRange::new(0, 5);
        range.set_start(6);
    }

    #[test]
    #[should_panic]
    fn set_invalid_range_end() {
        let mut range = ChannelRange::new(5, 5);
        range.set_end(4);
    }

    #[test]
    fn as_usize_range() {
        assert_eq!(ChannelRange::new(0, 5).as_range(), 0..=5);
        assert_eq!(ChannelRange::new(5, 5).as_range(), 5..=5);
    }
}
