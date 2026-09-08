use super::ast::Expr;
use super::lexer::Lexer;
use super::token::Tok;

struct Prs {
    ts: Vec<Tok>,
    p: usize,
}

impl Prs {
    fn new(s: &str) -> Self {
        Prs { ts: Lexer::new(s).all(), p: 0 }
    }

    fn cur(&self) -> &Tok {
        self.ts.get(self.p).unwrap_or(&Tok::Eof)
    }

    fn eat(&mut self) -> Tok {
        let t = self.cur().clone();
        self.p += 1;
        t
    }

    fn exp(&mut self, t: &Tok) {
        let g = self.eat();
        if &g != t {
            eprintln!("Expected {t:?}, got {g:?}");
            std::process::exit(1);
        }
    }

    fn expr(&mut self) -> Expr {
        self.add()
    }

    fn add(&mut self) -> Expr {
        let mut l = self.mul();
        loop {
            match self.cur() {
                Tok::Plus => { self.eat(); l = Expr::Add(Box::new(l), Box::new(self.mul())); }
                Tok::Minus => { self.eat(); l = Expr::Sub(Box::new(l), Box::new(self.mul())); }
                _ => break,
            }
        }
        l
    }

    fn mul(&mut self) -> Expr {
        let mut l = self.pow();
        loop {
            match self.cur() {
                Tok::Star => { self.eat(); l = Expr::Mul(Box::new(l), Box::new(self.pow())); }
                Tok::Slash => { self.eat(); l = Expr::Div(Box::new(l), Box::new(self.pow())); }
                _ => break,
            }
        }
        l
    }

    fn pow(&mut self) -> Expr {
        let b = self.una();
        if self.cur() == &Tok::Caret {
            self.eat();
            Expr::Pow(Box::new(b), Box::new(self.pow()))
        } else {
            b
        }
    }

    fn una(&mut self) -> Expr {
        match self.cur().clone() {
            Tok::Minus => { self.eat(); Expr::Neg(Box::new(self.atom())) }
            Tok::Plus => { self.eat(); self.atom() }
            _ => self.atom(),
        }
    }

    fn atom(&mut self) -> Expr {
        match self.cur().clone() {
            Tok::Num(n) => { self.eat(); Expr::Num(n) }
            Tok::Id(n) => {
                self.eat();
                if self.cur() == &Tok::LParen {
                    self.eat();
                    let mut a = Vec::new();
                    if self.cur() != &Tok::RParen {
                        a.push(self.expr());
                        while self.cur() == &Tok::Comma { self.eat(); a.push(self.expr()); }
                    }
                    self.exp(&Tok::RParen);
                    Expr::Call(n, a)
                } else {
                    Expr::Var(n)
                }
            }
            Tok::LParen => {
                self.eat();
                let e = self.expr();
                self.exp(&Tok::RParen);
                e
            }
            t => {
                eprintln!("Unexpected: {t:?}");
                std::process::exit(1);
            }
        }
    }
}

pub fn parse(s: &str) -> Expr {
    let mut p = Prs::new(s);
    let e = p.expr();
    if p.cur() != &Tok::Eof {
        eprintln!("Unexpected trailing: {:?}", p.cur());
        std::process::exit(1);
    }
    e
}
