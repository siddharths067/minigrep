use std::fs;
use std::env;


pub fn run(config: Config) -> Result<(), Box<std::error::Error>> {
	let contents = fs::read_to_string(config.filename)?;
	let results = if config.case_sensitive {
		search(config.query, &contents)
	}
	else {
		search_insensitive(config.query, &contents)
	};
	for line in results { 
		println!("{}", line);
	}
	Ok(())
}

pub struct Config<'a>{
	query: &'a String,
	filename: &'a String,
	case_sensitive: bool
}

impl Config<'_> {
	pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
		let query = args.get(1).ok_or("Query Not Specified")?;	
		let file = args.get(2).ok_or("File not specified")?;
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();	
	
		Ok(
			Config{
				query: query,
				filename: file,
				case_sensitive
			}
		)
	}

	pub fn query(&self) -> &String {
		return self.query;
	}
	
	pub fn filename(&self) -> &String {
		return self.filename;
	}
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut store: Vec<&str> = Vec::new();
	for line in contents.lines(){
		if line.contains(query) {
			store.push(line);
		}
	}
	store
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut store: Vec<&str> = Vec::new();
	for line in contents.lines(){
		if line.to_lowercase().contains(&query) {
			store.push(line);
		}
	}
	store
}
