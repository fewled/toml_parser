use std::io;

pub enum ParserError {
    FileNotFound,
    Unkown
}

impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> ParserError {
        match err.kind() {
            io::ErrorKind::NotFound => ParserError::FileNotFound,
            _ => ParserError::Unkown
        }
    }
}
