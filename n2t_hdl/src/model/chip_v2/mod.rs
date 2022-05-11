mod native;

#[derive(Clone)]
struct Id(u16);

trait Chip {
    fn clock(&mut self);
    fn eval(&mut self, args: &[bool]);
}