use super::token::Tok;

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(s: &str) -> Self {
        Lexer { src: s.chars().collect(), pos: 0 }
    }

    fn pk(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn adv(&mut self) -> Option<char> {
        let c = self.src.get(self.pos).copied();
        if c.is_some() { self.pos += 1; }
        c
    }

    fn skip(&mut self) {
        while let Some(c) = self.pk() {
            if c.is_ascii_whitespace() { self.adv(); } else { break; }
        }
    }

    pub fn next(&mut self) -> Tok {
        self.skip();
        let c = match self.adv() {
            Some(c) => c,
            None => return Tok::Eof,
        };
        match c {
            '+' => Tok::Plus,
            '-' => Tok::Minus,
            '*' => Tok::Star,
            '/' => Tok::Slash,
            '^' => Tok::Caret,
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            ',' => Tok::Comma,
            '0'..='9' => {
                let mut s = c.to_string();
                let mut dot = c == '.';
                while let Some(ch) = self.pk() {
                    if ch.is_ascii_digit() {
                        s.push(ch);
                        self.adv();
                    } else if ch == '.' && !dot {
                        dot = true;
                        s.push(ch);
                        self.adv();
                    } else {
                        break;
                    }
                }
                Tok::Num(s.parse::<f64>().unwrap_or_else(|_| {
                    eprintln!("Bad number: {s}");
                    std::process::exit(1);
                }))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut s = c.to_string();
                while let Some(ch) = self.pk() {
                    if ch.is_alphanumeric() || ch == '_' {
                        s.push(ch);
                        self.adv();
                    } else {
                        break;
                    }
                }
                Tok::Id(s)
            }
            _ => {
                eprintln!("Unexpected char: '{c}'");
                std::process::exit(1);
            }
        }
    }

    pub fn all(&mut self) -> Vec<Tok> {
        let mut v = Vec::new();
        loop {
            let t = self.next();
            let eof = t == Tok::Eof;
            v.push(t);
            if eof { break; }
        }
        v
    }
}
