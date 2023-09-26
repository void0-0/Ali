pub(crate) mod punch;

pub use punch::Punch;
use rand::prelude::*;

pub struct PunchFactory {
    state: bool
}

impl PunchFactory {
    pub fn new() -> PunchFactory {
        PunchFactory{ state: true }
    }

    pub fn generate_punch(&mut self) -> Punch {
        let mut rng = rand::thread_rng();

        let kind = if self.state {
            rng.gen_range(1..=3).into()
        } else {
            rng.gen_range(3..=5).into()
        };

        let target = rng.next_u32().into();

        self.state = !self.state;

        Punch { kind , target }
    }
}