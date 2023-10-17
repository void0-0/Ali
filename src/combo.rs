mod punch_factory;
pub mod format;

use punch_factory::PunchFactory;
use punch_factory::Punch;
use crate::config::Config;
use crate::combo::format::Format;
pub use punch_factory::punch::to_format::ToFormat;

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

impl<'a> ToFormat for Combo<'a> {
    fn to_format(&self, format: &Format) -> String {
        self.inner.iter().map(|punch| punch.to_format(format)).collect::<Vec<String>>().join(self.config.separator.as_str())
    }
}
