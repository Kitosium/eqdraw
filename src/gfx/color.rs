#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clr {
    Red,
    Grn,
    Yel,
    Blu,
    Mag,
    Cyn,
    Wht,
}

const PAL: [Clr; 6] = [Clr::Red, Clr::Grn, Clr::Yel, Clr::Blu, Clr::Mag, Clr::Cyn];

pub const RST: &str = "\x1b[0m";
pub const B: &str = "\x1b[1m";
pub const D: &str = "\x1b[2m";
pub const C: &str = "\x1b[36m";
pub const G: &str = "\x1b[90m";
pub const R: &str = "\x1b[31m";
pub const Y: &str = "\x1b[33m";

pub fn gt(i: usize) -> Clr {
    PAL[i % PAL.len()]
}

impl Clr {
    pub fn esc(self) -> &'static str {
        match self {
            Clr::Red => "\x1b[31m",
            Clr::Grn => "\x1b[32m",
            Clr::Yel => "\x1b[33m",
            Clr::Blu => "\x1b[34m",
            Clr::Mag => "\x1b[35m",
            Clr::Cyn => "\x1b[36m",
            Clr::Wht => "\x1b[37m",
        }
    }
}
