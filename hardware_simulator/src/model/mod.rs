

trait Chip {
    fn clock(&mut self);
    fn eval(&mut self, _: Vec<bool>) -> Vec<bool>;
}