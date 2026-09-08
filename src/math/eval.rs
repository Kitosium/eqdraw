use super::ast::Expr;
use super::ctx::Ctx;

pub fn ev(e: &Expr, c: &mut Ctx) -> Result<f64, String> {
    match e {
        Expr::Num(n) => Ok(*n),
        Expr::Var(n) => c.vs.get(n).copied().ok_or_else(|| format!("Unknown var: {n}")),
        Expr::Neg(x) => ev(x, c).map(|v| -v),
        Expr::Add(l, r) => Ok(ev(l, c)? + ev(r, c)?),
        Expr::Sub(l, r) => Ok(ev(l, c)? - ev(r, c)?),
        Expr::Mul(l, r) => Ok(ev(l, c)? * ev(r, c)?),
        Expr::Div(l, r) => {
            let b = ev(r, c)?;
            if b == 0.0 { Err("Division by zero".into()) } else { Ok(ev(l, c)? / b) }
        }
        Expr::Pow(b, e) => Ok(ev(b, c)?.powf(ev(e, c)?)),
        Expr::Call(n, a) => {
            if let Some(&f) = c.f1.get(n.as_str()) {
                if a.len() != 1 { return Err(format!("{n} needs 1 arg")); }
                Ok(f(ev(&a[0], c)?))
            } else if let Some(&f) = c.f2.get(n.as_str()) {
                if a.len() != 2 { return Err(format!("{n} needs 2 args")); }
                Ok(f(ev(&a[0], c)?, ev(&a[1], c)?))
            } else {
                Err(format!("Unknown func: {n}"))
            }
        }
    }
}
