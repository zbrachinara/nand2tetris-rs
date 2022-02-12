#[derive(Debug, Clone)]
pub enum ClockBehavior {
    Combinatorial,
    Sequential,
}

impl ClockBehavior {
    pub fn and(&self, rhs: &Self) -> Self {
        if matches!(self, ClockBehavior::Sequential) || matches!(rhs, ClockBehavior::Sequential) {
            ClockBehavior::Sequential
        } else {
            ClockBehavior::Combinatorial
        }
    }
}
