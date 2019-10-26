pub enum Token {
    Template(String),
    Text(String),
    Variable(String)
}

impl Token {
    pub fn text(contents: &str) -> Token {
        Token::Text(contents.to_owned())
    }

    pub fn variable(name: &str) -> Token {
        Token::Variable(name.to_owned())
    }

    pub fn template(name: &str) -> Token {
        Token::Template(name.to_owned())
    }
}