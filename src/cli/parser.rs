pub struct Args {
    pub qrs: Vec<String>,
    pub xmn: Option<f64>,
    pub xmx: Option<f64>,
    pub ymn: Option<f64>,
    pub ymx: Option<f64>,
}

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
                prn_help();
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("eqdraw {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "-q" | "--queries" => {
                i += 1;
                while i < av.len() && !av[i].starts_with('-') {
                    qrs.push(av[i].clone());
                    i += 1;
                }
                continue;
            }
            "--xmin" | "--xmax" | "--ymin" | "--ymax" => {
                let flg = av[i].clone();
                i += 1;
                if i >= av.len() {
                    eprintln!("Missing value for {flg}");
                    std::process::exit(1);
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
                eprintln!("Unknown option: {o}");
                eprintln!("Use --help for usage.");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if qrs.is_empty() {
        eprintln!("No equations. Use -q <equation>.");
        std::process::exit(1);
    }

    Args { qrs, xmn, xmx, ymn, ymx }
}

fn pv64(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or_else(|_| {
        eprintln!("Invalid number: {s}");
        std::process::exit(1);
    })
}

fn prn_help() {
    println!("eqdraw {}", env!("CARGO_PKG_VERSION"));
    println!("Plot EQUATIONS in your TERMINAL\n");
    println!("USAGE:");
    println!("    eqdraw [OPTIONS] -q <EQUATION>...\n");
    println!("OPTIONS:");
    println!("    -q, --queries <EQ>    Equations to plot (repeatable)");
    println!("    --xmin <VAL>          Min x-axis");
    println!("    --xmax <VAL>          Max x-axis");
    println!("    --ymin <VAL>          Min y-axis");
    println!("    --ymax <VAL>          Max y-axis");
    println!("    -h, --help            Help");
    println!("    -V, --version         Version\n");
    println!("FORMAT: y=<expr>   e.g. y=x^2, y=sin(x)\n");
    println!("FUNCS: sqrt exp abs pow root sin cos tan cot sec csc");
    println!("       asin acos atan acot asec acsc");
    println!("       sinh cosh tanh coth sech csch");
    println!("       asinh acosh atanh acoth asech acsch");
    println!("       ln log logb floor ceil round\n");
    println!("CONST: pi e");
}
