struct Parser {
    path: String,
    raw: String,
    tokens: Vec<&str>
}

enum ParserErr {
    FileNotFound,
    PermissionDenied
}

impl From<?> for ParserErr {}

impl Pasrer {
    pub fn load(path: String) -> Result<Self, ParserErr> {}
}


