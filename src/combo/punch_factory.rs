pub mod punch;
mod punch_factory_state;

pub use punch::{ Punch, punch_kind::PunchKind };
use rand::prelude::*;
use crate::combo::punch_factory::punch_factory_state::PunchFactoryState;
use crate::config::Config;
use crate::config::side::Side;

pub struct PunchFactory<'a> {
    state: PunchFactoryState,
    config: &'a Config
}

impl<'a> PunchFactory<'a> {
    pub fn new(config: &Config) -> PunchFactory {
        PunchFactory{
            state: PunchFactoryState::new(config.handedness.into()),
            config
        }
    }

    pub fn generate_punch(&mut self) -> Punch {
        let mut rng = rand::thread_rng();

        // Make it so that it can start with a jab
        // Make it so that it stays on a target for at least 2 hits
        let kind = match self.state.ready_side {
            Side::Right => PunchKind::gen_right_side_punch_kind(),
            _ => PunchKind::gen_left_side_punch_kind()
        };

        let target = rng.next_u32().into();

        self.state.switch_ready_side();

        Punch { kind , target }
    }
}