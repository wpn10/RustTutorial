use std::env;
use std::process;
use grepcli::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("error {}", err);
        process::exit(1);
    });
    println!("Searching {} in {}", config.query, config.filename);
    if let Err(e) = grepcli::run(config){
        println!("Error is {}", e);
    }

}
