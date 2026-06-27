use crate::{ Node, Store, ParserError };
use std::fs;

pub struct Slurp {
    raw: String,
    tokens: Vec<(usize, usize)>,
    store: Vec<Node>
}

impl Slurp {
    fn new(path: &str) -> Result<Self, ParserError> {
        Ok(Self {
            raw: fs::read_to_string(path)?,
            tokens: Vec::new(),
            store: Vec::new()
        })
    }
    pub fn parse(path: &str) -> Result<Store, ParserError> {
        let parser = Slurp::new(path)?;
        Ok(Store::Slurp(parser))
    }
}
