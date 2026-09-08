mod cli;
mod gfx;
mod math;
mod plot;

use cli::parser::parse;
use gfx::color::RST;
use plot::render::run;

fn main() {
    let a = parse();
    if let Err(e) = run(a) {
        eprintln!("\x1b[31m!{RST} {e}");
        std::process::exit(1);
    }
}
