use super::ast::Expr;
use super::ctx::Ctx;

const NR0: f64 = 1e-100;
const HUGE: f64 = 1e100;

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
            if b.abs() < NR0 {
                Err("Division by zero".into())
            } else {
                let a = ev(l, c)?;
                let q = a / b;
                if q.abs() > HUGE {
                    Err("Overflow".into())
                } else {
                    Ok(q)
                }
            }
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
