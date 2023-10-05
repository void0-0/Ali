pub mod punch;
mod punch_factory_state;

pub use punch::{ Punch, punch_kind::PunchKind, punch_target::PunchTarget };
use crate::combo::punch_factory::punch_factory_state::PunchFactoryState;
use crate::config::Config;

pub struct PunchFactory<'a> {
    state: PunchFactoryState,
    config: &'a Config
}

impl<'a> PunchFactory<'a> {
    pub fn new(config: &Config) -> PunchFactory {
        PunchFactory{
            state: PunchFactoryState::new(config.stance.into()),
            config
        }
    }

    pub fn generate_punch(&mut self) -> Punch {
        let punch = Punch {
            kind: match self.config.allow_same_side_punch {
                true => PunchKind::gen_random_punch_kind(),
                false => PunchKind::gen_punch_kind(self.state.punch_threw_count == 0, self.state.ready_side)
            },
            target: PunchTarget::gen_target(self.config.allow_head_punch, self.config.allow_body_punch)
        };

        self.tick();

        punch
    }

    fn tick(&mut self) {
        self.state.switch_ready_side();
        self.state.punch_threw_count += 1;
    }
}