use crate::gfx::buf::Buf;
use crate::gfx::color::Clr;

pub fn ln(b: &mut Buf, p1: (i32, i32), p2: (i32, i32), c: Clr) {
    let (mut x1, mut y1) = p1;
    let (x2, y2) = p2;
    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x1 >= 0 && x1 < b.w as i32 && y1 >= 0 && y1 < b.h as i32 {
            b.set(x1 as usize, y1 as usize, c);
        }
        if x1 == x2 && y1 == y2 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x1 += sx; }
        if e2 <= dx { err += dx; y1 += sy; }
    }
}

pub fn ax(b: &mut Buf, xmn: f64, xmx: f64, ymn: f64, ymx: f64) {
    if xmn <= 0.0 && xmx >= 0.0 {
        let xz = ((0.0 - xmn) / (xmx - xmn) * (b.w - 1) as f64).round() as usize;
        if xz < b.w {
            for y in 0..b.h { b.set0(xz, y, Clr::Wht); }
        }
    }
    if ymn <= 0.0 && ymx >= 0.0 {
        let yz = ((ymx - 0.0) / (ymx - ymn) * (b.h - 1) as f64).round() as usize;
        if yz < b.h {
            for x in 0..b.w { b.set0(x, yz, Clr::Wht); }
        }
    }
}
