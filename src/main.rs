mod cli;
mod gfx;
mod math;
mod plot;

use cli::parser::parse;
use plot::render::run;

fn main() {
    let a = parse();
    if let Err(e) = run(a) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
