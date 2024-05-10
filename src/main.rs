mod args;
mod error;
mod program;
mod utils;

fn main() {
    let result = program::run();

    if let Err(err) = result {
        eprintln!("{}", err);
    }
}
