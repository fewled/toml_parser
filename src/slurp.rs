use crate::Node;

pub struct Slurp {
    raw: String,
    tokens: Vec<(usize, usize)>,
    store: Vec<Node>
}

