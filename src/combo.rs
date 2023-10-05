mod punch_factory;

use punch_factory::PunchFactory;
use punch_factory::Punch;
pub use punch_factory::punch::to_boxer_format::ToBoxerFormat;
use crate::config::Config;

pub struct Combo<'a> {
    inner: Vec<Punch>,
    config: &'a Config
}

impl<'a> Combo<'a> {
    pub fn new(config: &'a Config) -> Combo<'a> {
        let mut factory = PunchFactory::new(&config);

        Combo { inner: (0..config.length).map(|_| factory.generate_punch()).collect::<Vec<_>>(), config }
    }
}

impl<'a> ToBoxerFormat for Combo<'a> {
    fn to_boxer_format(&self) -> String {
        self.inner.iter().map(|punch| punch.to_boxer_format()).collect::<Vec<String>>().join(self.config.separator.as_str())
    }
}
