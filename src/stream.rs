use crate::Node;

pub struct Stream {
    line: String,
    tokens: Vec<String>,
    store: Vec<Node>
}
