use super::ast::Expr;
use super::lexer::Lexer;
use super::token::Tok;

const MAX_D: usize = 128;

struct Prs {
    ts: Vec<Tok>,
    p: usize,
    d: usize,
}

impl Prs {
    fn new(s: &str) -> Result<Self, String> {
        Ok(Prs { ts: Lexer::new(s).all()?, p: 0, d: 0 })
    }

    fn cur(&self) -> &Tok {
        self.ts.get(self.p).unwrap_or(&Tok::Eof)
    }

    fn eat(&mut self) -> Tok {
        let t = self.cur().clone();
        self.p += 1;
        t
    }

    fn chk(&mut self, t: &Tok) -> Result<(), String> {
        let g = self.eat();
        if &g != t {
            Err(format!("Expected {t:?}, got {g:?}"))
        } else {
            Ok(())
        }
    }

    fn enter(&mut self) -> Result<(), String> {
        if self.d >= MAX_D {
            return Err("Expression too deeply nested".into());
        }
        self.d += 1;
        Ok(())
    }

    fn leave(&mut self) {
        self.d -= 1;
    }

    fn expr(&mut self) -> Result<Expr, String> {
        self.enter()?;
        let r = self.add();
        self.leave();
        r
    }

    fn add(&mut self) -> Result<Expr, String> {
        let mut l = self.mul()?;
        loop {
            match self.cur() {
                Tok::Plus => { self.eat(); l = Expr::Add(Box::new(l), Box::new(self.mul()?)); }
                Tok::Minus => { self.eat(); l = Expr::Sub(Box::new(l), Box::new(self.mul()?)); }
                _ => break,
            }
        }
        Ok(l)
    }

    fn mul(&mut self) -> Result<Expr, String> {
        let mut l = self.pow()?;
        loop {
            match self.cur() {
                Tok::Star => {
                    self.eat();
                    l = Expr::Mul(Box::new(l), Box::new(self.pow()?));
                }
                Tok::Slash => {
                    self.eat();
                    l = Expr::Div(Box::new(l), Box::new(self.pow()?));
                }
                Tok::Id(_) | Tok::LParen => {
                    l = Expr::Mul(Box::new(l), Box::new(self.pow()?));
                }
                _ => break,
            }
        }
        Ok(l)
    }

    fn pow(&mut self) -> Result<Expr, String> {
        let b = self.atom()?;
        if self.cur() == &Tok::Caret {
            self.eat();
            let e = self.pow()?;
            Ok(Expr::Pow(Box::new(b), Box::new(e)))
        } else {
            Ok(b)
        }
    }

    fn atom(&mut self) -> Result<Expr, String> {
        match self.cur().clone() {
            Tok::Minus => {
                self.eat();
                let e = self.atom()?;
                Ok(Expr::Neg(Box::new(e)))
            }
            Tok::Plus => {
                self.eat();
                self.atom()
            }
            Tok::Num(n) => {
                self.eat();
                Ok(Expr::Num(n))
            }
            Tok::Id(n) => {
                self.eat();
                if self.cur() == &Tok::LParen {
                    self.eat();
                    let mut a = Vec::new();
                    if self.cur() != &Tok::RParen {
                        a.push(self.expr()?);
                        while self.cur() == &Tok::Comma {
                            self.eat();
                            a.push(self.expr()?);
                        }
                    }
                    self.chk(&Tok::RParen)?;
                    Ok(Expr::Call(n, a))
                } else {
                    Ok(Expr::Var(n))
                }
            }
            Tok::LParen => {
                self.eat();
                let e = self.expr()?;
                self.chk(&Tok::RParen)?;
                Ok(e)
            }
            t => Err(format!("Unexpected: {t:?}")),
        }
    }
}

pub fn parse(s: &str) -> Expr {
    match prs(s) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Parse error: {e}");
            std::process::exit(1);
        }
    }
}

fn prs(s: &str) -> Result<Expr, String> {
    let mut p = Prs::new(s)?;
    let e = p.expr()?;
    if p.cur() != &Tok::Eof {
        return Err(format!("Unexpected trailing: {:?}", p.cur()));
    }
    Ok(e)
}
