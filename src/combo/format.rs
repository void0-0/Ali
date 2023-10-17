use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Format {
    Normal,
    Boxer
}