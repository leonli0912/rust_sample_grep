use std::fs;
use std::error::Error;

pub struct Config{
    pub query:String,
    pub filename:String,
}  

impl Config {
    pub fn new(args:&[String]) -> Result<Config,&str>{
        if args.len()<3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config {
            query,
            filename
        })
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    //.expect("somthing went wrong on reading the file");
    for line in search(&config.query,&contents){
        println!("{}",line);
    }
    Ok(())
}

pub fn search<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_reuslt(){
        let query = "duct";
        let contents = "\
        rust:safe,fast,productive.
        pcik three";

        assert_eq!(vec!["rust:safe,fast,productive."],search(query,contents));
    }
}