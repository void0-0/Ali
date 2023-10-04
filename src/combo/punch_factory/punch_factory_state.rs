use crate::config::side::Side;

pub struct PunchFactoryState {
    pub ready_side: Side,
    pub punch_threw_count: i32
}

impl PunchFactoryState {
    pub fn new(side: Side) -> PunchFactoryState {
        PunchFactoryState {
            ready_side: side,
            punch_threw_count: 0
        }
    }

    pub fn switch_ready_side(&mut self) {
        self.ready_side = !self.ready_side;
    }
}