use std::process;
use std::env;

use minigrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Applicantion error: {e}");
        process::exit(1);
    }

    // eprintln prints to standard error, instead of standard output,
    // this means that error messages will be processed as errors,
    // not as data (i.e. wouldn't be written in a file output).

}


