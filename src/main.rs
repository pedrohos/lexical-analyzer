use std::env;
use std::process;
use lexical_analyzer::common::config::Config;
use lexical_analyzer::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(
        |e| {
            println!("Error getting config: {e}");
            process::exit(1);
        });

    if let Err(e) = run(config) {
        println!("Error happened during run: {e}");
        process::exit(1);
    }
}
