use crate::cli::parser::Args;
use crate::gfx::braille::{self, DOTS};
use crate::gfx::buf::Buf;
use crate::gfx::color::{self, Clr, RST};
use crate::math::ast::Expr;
use crate::math::ctx::Ctx;
use crate::math::eval::ev;
use crate::math::parser::parse as mparse;
use crate::plot::draw;
use crate::plot::range;

const W: usize = 160;
const H: usize = 80;

pub fn run(a: Args) -> Result<(), String> {
    let mut cx = Ctx::new();
    let mut es: Vec<(Expr, usize)> = Vec::new();

    for (i, q) in a.qrs.iter().enumerate() {
        let eq = q.strip_prefix("y=")
            .ok_or_else(|| format!("Must start with 'y=': {q}"))?;
        let eq = eq.replace('√', "sqrt");
        es.push((mparse(&eq), i));
    }

    let bb = range::bnd(&es, &mut cx);

    if bb.is_none() && a.xmn.is_none() && a.xmx.is_none() {
        eprintln!("Cannot determine range. Use --xmin/--xmax.");
        return Ok(());
    }

    let (xmn, xmx, ymn, ymx) = range::rng(a.xmn, a.xmx, a.ymn, a.ymx, bb);
    let mut bf = Buf::new(W, H);

    for (e, i) in &es {
        pl(&mut bf, e, &mut cx, color::gt(*i), xmn, xmx, ymn, ymx)?;
    }

    draw::ax(&mut bf, xmn, xmx, ymn, ymx);
    prn(&bf, &a.qrs);
    Ok(())
}

fn pl(
    b: &mut Buf, e: &Expr, c: &mut Ctx, cl: Clr,
    xmn: f64, xmx: f64, ymn: f64, ymx: f64,
) -> Result<(), String> {
    let mut pre: Option<(i32, i32)> = None;

    for i in 0..W {
        let x = xmn + (i as f64 / (W - 1) as f64) * (xmx - xmn);
        c.vs.insert("x".into(), x);

        let y = match ev(e, c) {
            Ok(v) => v,
            Err(_) => { pre = None; continue; }
        };

        if !y.is_finite() { pre = None; continue; }

        let jf = (ymx - y) / (ymx - ymn) * (H - 1) as f64;
        if jf.abs() > 1_000_000.0 { pre = None; continue; }

        let j = jf.round() as i32;
        let cur = (i as i32, j);

        if let Some(pr) = pre { draw::ln(b, pr, cur, cl); }
        pre = Some(cur);
    }
    Ok(())
}

fn prn(b: &Buf, qs: &[String]) {
    for y in (0..H).step_by(4) {
        for x in (0..W).step_by(2) {
            let mut bits: u8 = 0;
            let mut cls = [None; 8];

            for (k, &(dy, dx)) in DOTS.iter().enumerate() {
                let ny = y + dy as usize;
                let nx = x + dx as usize;
                if nx < W && ny < H {
                    if let Some(c) = b.get(nx, ny) {
                        bits |= 1 << k;
                        cls[k] = Some(c);
                    }
                }
            }

            let ch = braille::enc(bits);
            let cl = cls.iter().filter_map(|&c| c)
                .find(|&c| c != Clr::Wht)
                .or_else(|| cls.iter().filter_map(|&c| c).next());

            if let Some(c) = cl {
                print!("{}{}{}", c.esc(), ch, RST);
            } else {
                print!("{ch}");
            }
        }
        println!();
    }

    println!("\nEquations:");
    for (i, q) in qs.iter().enumerate() {
        let c = color::gt(i);
        println!("• {}{}{}", c.esc(), q, RST);
    }
}
