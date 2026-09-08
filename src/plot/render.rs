use crate::cli::parser::Args;
use crate::gfx::braille::{self, DOTS};
use crate::gfx::buf::Buf;
use crate::gfx::color::{self, B, G, RST};
use crate::math::ast::Expr;
use crate::math::ctx::Ctx;
use crate::math::eval::ev;
use crate::math::parser::parse as mparse;
use crate::plot::draw;
use crate::plot::range;

const W: usize = 160;
const H: usize = 80;
const ASP: f64 = 0.8;

pub fn run(a: Args) -> Result<(), String> {
    let mut cx = Ctx::new();
    let mut es: Vec<(Expr, usize)> = Vec::new();

    for (i, q) in a.qrs.iter().enumerate() {
        let eq = q.strip_prefix("y=")
            .ok_or_else(|| format!("Must start with 'y=': {q}"))?;
        let eq = eq.replace('√', "sqrt");
        es.push((mparse(&eq), i));
    }

    let bb = range::bnd(&es, &mut cx, a.xmn, a.xmx);

    if bb.is_none() && a.xmn.is_none() && a.xmx.is_none() {
        eprintln!("\x1b[33m!{RST} Cannot determine range. Use {B}--xmin{RST}/{B}--xmax{RST}.");
        return Ok(());
    }

    let (xmn, xmx, ymn, ymx) = range::rng(a.xmn, a.xmx, a.ymn, a.ymx, bb);
    let mut bf = Buf::new(W, H);

    let bd = Bnd { xmn, xmx, ymn, ymx };
    for (e, i) in &es {
        pl(&mut bf, e, &mut cx, color::gt(*i), &bd)?;
    }

    draw::ax(&mut bf, xmn, xmx, ymn, ymx);
    prn(&bf, &a.qrs, es.len());
    Ok(())
}

struct Bnd { xmn: f64, xmx: f64, ymn: f64, ymx: f64 }

fn pl(
    b: &mut Buf, e: &Expr, c: &mut Ctx, cl: crate::gfx::color::Clr,
    bd: &Bnd,
) -> Result<(), String> {
    let mut pre: Option<(i32, i32)> = None;
    let mut pre_y: Option<f64> = None;
    let yspan = (bd.ymx - bd.ymn).abs();

    for i in 0..W {
        let x = bd.xmn + (i as f64 / (W - 1) as f64) * (bd.xmx - bd.xmn);
        c.vs.insert("x".into(), x);

        let y = match ev(e, c) {
            Ok(v) => v,
            Err(_) => { pre = None; pre_y = None; continue; }
        };

        if !y.is_finite() { pre = None; pre_y = None; continue; }

        if let Some(py) = pre_y {
            if (y - py).abs() > yspan * ASP {
                pre = None;
            }
        }

        let jf = (bd.ymx - y) / (bd.ymx - bd.ymn) * (H - 1) as f64;
        if jf.abs() > 1_000_000.0 { pre = None; pre_y = Some(y); continue; }

        let j = jf.round() as i32;
        let cur = (i as i32, j);

        if let Some(pr) = pre { draw::ln(b, pr, cur, cl); }
        pre = Some(cur);
        pre_y = Some(y);
    }
    Ok(())
}

fn prn(b: &Buf, qs: &[String], n: usize) {
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
                .find(|&c| c != crate::gfx::color::Clr::Wht)
                .or_else(|| cls.iter().filter_map(|&c| c).next());

            if let Some(c) = cl {
                print!("{}{}{}", c.esc(), ch, RST);
            } else {
                print!("{ch}");
            }
        }
        println!();
    }

    println!();
    println!("  {B}EQUATIONS{RST}");
    for (i, q) in qs.iter().enumerate().take(n) {
        let cl = color::gt(i).esc();
        println!("    {G}{}{RST}  {}{}{RST}", i + 1, cl, q);
    }
    println!();
}
