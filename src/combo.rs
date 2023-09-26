mod punch_factory;

use punch_factory::PunchFactory;
use punch_factory::Punch;
pub use punch_factory::punch::to_boxer_format::ToBoxerFormat;

pub struct Combo {
    inner: Vec<Punch>
}

impl Combo {
    pub fn new(length: i32) -> Combo {
        let mut factory = PunchFactory::new();

        Combo { inner: (0..length).map(|_| factory.generate_punch()).collect::<Vec<_>>() }
    }
}

impl ToBoxerFormat for Combo {
    fn to_boxer_format(&self) -> String {
        self.inner.iter().map(|punch| punch.to_boxer_format()).collect::<Vec<String>>().join(", ")
    }
}
