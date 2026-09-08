use crate::math::ast::Expr;
use crate::math::ctx::Ctx;
use crate::math::eval::ev;

pub struct BBox {
    pub xmn: f64,
    pub xmx: f64,
    pub ymn: f64,
    pub ymx: f64,
}

const DX: f64 = -10.0;
const MX: f64 = 10.0;
const DY: f64 = -10.0;
const MY: f64 = 10.0;
const ASP: f64 = 2.0;

pub fn bnd(es: &[(Expr, usize)], c: &mut Ctx, xmn: Option<f64>, xmx: Option<f64>) -> Option<BBox> {
    let mut pts: Vec<(f64, f64)> = Vec::new();
    let lo = xmn.unwrap_or(DX);
    let hi = xmx.unwrap_or(MX);
    let n = 200;
    let sp: Vec<f64> = (0..=n).map(|i| lo + (hi - lo) * i as f64 / n as f64).collect();

    for (e, _) in es {
        for &x in &sp {
            c.vs.insert("x".into(), x);
            if let Ok(y) = ev(e, c) {
                if y.is_finite() { pts.push((x, y)); }
            }
        }
    }

    if pts.is_empty() { return None; }

    let mut b = BBox { xmn: pts[0].0, xmx: pts[0].0, ymn: pts[0].1, ymx: pts[0].1 };
    for &(x, y) in &pts[1..] {
        b.xmn = b.xmn.min(x);
        b.xmx = b.xmx.max(x);
        b.ymn = b.ymn.min(y);
        b.ymx = b.ymx.max(y);
    }
    Some(b)
}

pub fn rng(
    amn: Option<f64>, amx: Option<f64>,
    bmn: Option<f64>, bmx: Option<f64>,
    bb: Option<BBox>,
) -> (f64, f64, f64, f64) {
    let (mut fmn, mut fmx, mut gmn, mut gmx);

    if let Some(b) = bb {
        fmn = amn.unwrap_or(b.xmn - 1.0);
        fmx = amx.unwrap_or(b.xmx + 1.0);
        gmn = bmn.unwrap_or(b.ymn - 1.0);
        gmx = bmx.unwrap_or(b.ymx + 1.0);

        if bmn.is_none() && bmx.is_none() {
            let xs = fmx - fmn;
            let ys = gmx - gmn;
            let w = xs / ASP;
            if ys > w {
                if (gmn < 0.0) != (gmx < 0.0) {
                    gmn = -w / 2.0;
                    gmx = w / 2.0;
                } else if gmn.abs() < gmx.abs() {
                    gmx = gmn + w;
                } else {
                    gmn = gmx - w;
                }
            }
        }
    } else {
        fmn = amn.unwrap_or(DX);
        fmx = amx.unwrap_or(MX);
        gmn = bmn.unwrap_or(DY);
        gmx = bmx.unwrap_or(MY);
    }

    if (fmx - fmn).abs() < f64::EPSILON { fmn -= 1.0; fmx += 1.0; }
    if (gmx - gmn).abs() < f64::EPSILON { gmn -= 1.0; gmx += 1.0; }

    if amn.is_none() { fmn = fmn.min(0.0); }
    if amx.is_none() { fmx = fmx.max(0.0); }
    if bmn.is_none() { gmn = gmn.min(0.0); }
    if bmx.is_none() { gmx = gmx.max(0.0); }

    (fmn, fmx, gmn, gmx)
}
