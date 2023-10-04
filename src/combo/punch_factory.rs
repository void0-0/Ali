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

        let kind = if self.state.punch_threw_count != 0 {
            match self.state.ready_side {
                Side::Right => PunchKind::gen_right_side_punch_kind(),
                _ => PunchKind::gen_left_side_punch_kind()
            }
        } else {
            PunchKind::gen_first_punch_kind()
        };

        let target = rng.next_u32().into();

        self.state.switch_ready_side();

        Punch { kind , target }
    }
}