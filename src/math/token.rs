#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Num(f64),
    Id(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
    Eof,
}
