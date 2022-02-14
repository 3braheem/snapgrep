use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Two arguments are required: [query] [filename]");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("ANY_CASE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_any_case(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_any_case<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_config() {
        let args = vec!["debug".to_owned(), "test".to_owned(), "poem.txt".to_owned()];
        let config = Config::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing args: {}", err);
            std::process::exit(1);
        });
        assert_eq!(config.filename, "poem.txt");
        assert_eq!(config.query, "test");
    }

    #[test]
    fn run_config() {
        let args = vec!["debug".to_owned(), "test".to_owned(), "poem.txt".to_owned()];
        let config = Config::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing args: {}", err);
            std::process::exit(1);
        });
        let success = run(config).unwrap();
        assert_eq!(success, ());
    }

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn any_case() {
        let query = "rUsT";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust: ", "Trust me."],
            search_any_case(query, contents)
        );
    }
}
