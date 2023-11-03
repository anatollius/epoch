use epoch::{convert_input, get_inputs};
use std::env;
use std::process;
fn main() {
    let inputs = get_inputs(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    for input in inputs {
        let output = convert_input(&input).unwrap_or_else(|err| {
            eprintln!("{err}: {input}");
            process::exit(1)
        });

        println!("{output}")
    }
}
