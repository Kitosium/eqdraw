use super::color::Clr;

pub struct Buf {
    pub w: usize,
    pub h: usize,
    pub px: Vec<Vec<Option<Clr>>>,
}

impl Buf {
    pub fn new(w: usize, h: usize) -> Self {
        Buf { w, h, px: vec![vec![None; w]; h] }
    }

    pub fn set(&mut self, x: usize, y: usize, c: Clr) {
        if x < self.w && y < self.h { self.px[y][x] = Some(c); }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Clr> {
        if x < self.w && y < self.h { self.px[y][x] } else { None }
    }

    pub fn set0(&mut self, x: usize, y: usize, c: Clr) {
        if x < self.w && y < self.h && self.px[y][x].is_none() {
            self.px[y][x] = Some(c);
        }
    }
}
