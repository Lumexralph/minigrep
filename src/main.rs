use std::env;
use std::process;

use minigrep;
use minigrep::Config;


fn main() {
    // get the arguments from the command line
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1)
    });

    if let Err(e) =  minigrep::run(config) {
        // send error to the stderr stream using eprintln macro
        eprintln!("Application error {}", e);

        process::exit(1);
    };

}
