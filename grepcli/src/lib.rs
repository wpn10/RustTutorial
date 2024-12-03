use std::error::Error;
use std::fs;

pub struct Config{
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() <3{
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    let results = search(&config.query, &content);
    for result in results{
        println!("present in this line {}", result);
    }

    Ok(())
}


pub fn search<'a>(query: &str, file: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in file.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
