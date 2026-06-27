use crate::{ Slurp, Stream };

pub enum Store {
    Slurp(Slurp),
    Stream(Stream)
}
