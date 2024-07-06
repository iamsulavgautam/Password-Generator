use serde::Deserialize;

#[derive(Deserialize)]
pub struct Selections {
    pub length: Option<usize>,
    pub put_numbers: Option<bool>,
    pub put_symbols: Option<bool>,
}