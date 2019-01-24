use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
            Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive,
            })
        }
    }
}

#[cfg(test)]
mod test_config {
    use super::Config;
    #[test]
    fn new_errors_with_less_than_three_args() {
        let args = vec![String::from("1"), String::from("2")];
        assert_eq!(Err("not enough arguments"), Config::new(&args));
    }
    #[test]
    fn sets_query_and_filename_correctly() {
        let args = vec![
            String::from("ignored"),
            String::from("query"),
            String::from("filename"),
        ];
        assert_eq!(
            Ok(Config {
                query: String::from("query"),
                filename: String::from("filename"),
                case_sensitive: true,
            }),
            Config::new(&args)
        );
    }

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let search_results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in search_results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive.",], search(&query, &contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(&query, &contents)
        );
    }

}
