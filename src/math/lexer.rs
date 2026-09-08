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

    fn save(&self) -> usize {
        self.pos
    }

    fn rst(&mut self, p: usize) {
        self.pos = p;
    }

    fn skip(&mut self) {
        while let Some(c) = self.pk() {
            if c.is_ascii_whitespace() { self.adv(); } else { break; }
        }
    }

    pub fn next(&mut self) -> Result<Tok, String> {
        self.skip();
        let c = match self.adv() {
            Some(c) => c,
            None => return Ok(Tok::Eof),
        };
        match c {
            '+' => Ok(Tok::Plus),
            '-' => Ok(Tok::Minus),
            '*' => Ok(Tok::Star),
            '/' => Ok(Tok::Slash),
            '^' => Ok(Tok::Caret),
            '(' => Ok(Tok::LParen),
            ')' => Ok(Tok::RParen),
            ',' => Ok(Tok::Comma),
            '0'..='9' | '.' => {
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
                let sv = self.save();
                if let Some(ch) = self.pk() {
                    if ch == 'e' || ch == 'E' {
                        self.adv();
                        let sv2 = self.save();
                        if let Some(sign) = self.pk() {
                            if sign == '+' || sign == '-' {
                                self.adv();
                            }
                        }
                        match self.pk() {
                            Some(d) if d.is_ascii_digit() => {
                                s.push('e');
                                if let Some(sign) = self.src.get(sv2) {
                                    if *sign == '+' || *sign == '-' {
                                        s.push(*sign);
                                    }
                                }
                                while let Some(ch) = self.pk() {
                                    if ch.is_ascii_digit() {
                                        s.push(ch);
                                        self.adv();
                                    } else {
                                        break;
                                    }
                                }
                            }
                            _ => {
                                self.rst(sv);
                            }
                        }
                    }
                }
                let val = s.parse::<f64>().map_err(|_| format!("Bad number: {s}"))?;
                Ok(Tok::Num(val))
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
                Ok(Tok::Id(s))
            }
            _ => Err(format!("Unexpected char: '{c}'")),
        }
    }

    pub fn all(&mut self) -> Result<Vec<Tok>, String> {
        let mut v = Vec::new();
        loop {
            let t = self.next()?;
            let eof = t == Tok::Eof;
            v.push(t);
            if eof { break; }
        }
        Ok(v)
    }
}
