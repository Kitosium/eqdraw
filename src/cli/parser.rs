use crate::gfx::color::{B, C, D, G, R, RST, Y};

const BANR: &str = "███████  ██████   ██████  ██████   █████  ██     ██\n██      ██    ██  ██   ██ ██   ██ ██   ██ ██     ██\n█████   ██    ██  ██   ██ ██████  ███████ ██  █  ██\n██      ██  ████  ██   ██ ██   ██ ██   ██ ██ ███ ██\n███████  ██████ █ ██████  ██   ██ ██   ██  ███ ███";

pub struct Args {
    pub qrs: Vec<String>,
    pub xmn: Option<f64>,
    pub xmx: Option<f64>,
    pub ymn: Option<f64>,
    pub ymx: Option<f64>,
}

const FLGS: &[&str] = &["-h", "--help", "-v", "--version", "-q", "--queries",
    "--xmin", "--xmax", "--ymin", "--ymax"];

pub fn parse() -> Args {
    let av: Vec<String> = std::env::args().skip(1).collect();
    let mut qrs = Vec::new();
    let mut xmn = None;
    let mut xmx = None;
    let mut ymn = None;
    let mut ymx = None;

    let mut i = 0;
    while i < av.len() {
        match av[i].as_str() {
            "-h" | "--help" => {
                phlp();
                std::process::exit(0);
            }
            "-v" | "--version" => {
                pver();
                std::process::exit(0);
            }
            "-q" | "--queries" => {
                i += 1;
                while i < av.len() && !FLGS.contains(&av[i].as_str()) {
                    qrs.push(av[i].clone());
                    i += 1;
                }
                continue;
            }
            "--xmin" | "--xmax" | "--ymin" | "--ymax" => {
                let flg = av[i].clone();
                i += 1;
                if i >= av.len() {
                    err(&format!("Missing value for {flg}"));
                }
                let v = pv64(&av[i]);
                match flg.as_str() {
                    "--xmin" => xmn = Some(v),
                    "--xmax" => xmx = Some(v),
                    "--ymin" => ymn = Some(v),
                    "--ymax" => ymx = Some(v),
                    _ => unreachable!(),
                }
            }
            o => {
                err(&format!("Unknown option: {o}\n       Use {B}--help{RST} for usage."));
            }
        }
        i += 1;
    }

    if qrs.is_empty() {
        err("No equations provided.\n       Use {B}-q{RST} <equation> to specify one.");
    }

    Args { qrs, xmn, xmx, ymn, ymx }
}

fn pv64(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or_else(|_| {
        err(&format!("Invalid number: {Y}{s}{RST}"));
    })
}

fn err(msg: &str) -> ! {
    eprintln!("{R}!{RST} {msg}");
    std::process::exit(1);
}

fn vis(s: &str) -> usize {
    let mut n = 0;
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '\x1b' {
            while let Some(&b) = it.as_str().as_bytes().first() {
                if b == b'm' { it.next(); break; }
                it.next();
            }
        } else {
            n += 1;
        }
    }
    n
}

fn pad(s: &str, w: usize) -> String {
    let v = vis(s);
    if v >= w { return s.to_string(); }
    format!("{}{}", s, " ".repeat(w - v))
}

fn pver() {
    println!();
    println!("\x1b[37m{BANR}\x1b[0m   \x1b[90mv{}\x1b[0m", env!("CARGO_PKG_VERSION"));
    println!();
    println!("{D}Plot equations in your terminal{RST}");
    println!("{G}https://github.com/Kitosium/eqdraw{RST}");
    println!();
}

fn phlp() {
    let v = env!("CARGO_PKG_VERSION");
    println!();
    println!("\x1b[37m{BANR}\x1b[0m   \x1b[90mv{v}\x1b[0m");
    println!();
    println!("  {D}Plot EQUATIONS in your TERMINAL{RST}");
    println!();
    println!("  {B}USAGE{RST}");
    println!("    {B}eqdraw{RST} {G}[OPTIONS]{RST} -q {C}<EQUATION>...{RST}");
    println!();
    println!("  {B}OPTIONS{RST}");
    println!("    {} Equations to plot {G}(repeatable){RST}", pad(&format!("{C}-q{RST}, {C}--queries{RST} {G}<EQ>{RST}"), 20));
    println!("    {} Min x-axis value", pad(&format!("{C}--xmin{RST} {G}<VAL>{RST}"), 20));
    println!("    {} Max x-axis value", pad(&format!("{C}--xmax{RST} {G}<VAL>{RST}"), 20));
    println!("    {} Min y-axis value", pad(&format!("{C}--ymin{RST} {G}<VAL>{RST}"), 20));
    println!("    {} Max y-axis value", pad(&format!("{C}--ymax{RST} {G}<VAL>{RST}"), 20));
    println!("    {} Show this help", pad(&format!("{C}-h{RST}, {C}--help{RST}"), 20));
    println!("    {} Show version", pad(&format!("{C}-v{RST}, {C}--version{RST}"), 20));
    println!();
    println!("  {B}FORMAT{RST}");
    println!("    {G}y={RST}{B}<expression>{RST}");
    println!();
    println!("    {} {G}parabola{RST}", pad(&format!("{C}y=x^2{RST}"), 12));
    println!("    {} {G}sine wave{RST}", pad(&format!("{C}y=sin(x){RST}"), 12));
    println!("    {} {G}linear{RST}", pad(&format!("{C}y=2x+1{RST}"), 12));
    println!("    {} {G}hyperbola{RST}", pad(&format!("{C}y=1/x{RST}"), 12));
    println!("    {} {G}exponential{RST}", pad(&format!("{C}y=e^x{RST}"), 12));
    println!();
    println!("  {B}FUNCTIONS{RST}");
    println!("    {D}sqrt exp abs pow root floor ceil round{RST}");
    println!("    {D}sin cos tan cot sec csc{RST}");
    println!("    {D}asin acos atan acot asec acsc{RST}");
    println!("    {D}sinh cosh tanh coth sech csch{RST}");
    println!("    {D}asinh acosh atanh acoth asech acsch{RST}");
    println!("    {D}ln log logb{RST}");
    println!();
    println!("  {B}CONSTANTS{RST}");
    println!("    {C}pi{RST} {D}\u{03C0} \u{2248} 3.14159{RST}  {C}e{RST} {D}Euler \u{2248} 2.71828{RST}");
    println!();
    println!("  {G}https://github.com/Kitosium/eqdraw{RST}");
    println!();
}
