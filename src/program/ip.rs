use data::{Value, Point, Delta};
use data::stack::StackStack;

pub struct Ip {
    id: Value,
    position: Point,
    velocity: Delta,
    storage: Delta,
    stacks: StackStack,
}
