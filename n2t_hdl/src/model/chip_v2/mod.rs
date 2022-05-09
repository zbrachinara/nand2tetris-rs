
trait Chip {
    fn clock(&mut self);
    fn eval(&mut self, args: &[bool]);
}