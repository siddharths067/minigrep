use std::env;

use minigrep::Config;
use minigrep::run;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let config = Config::new(&args)?;
    //println!("The Query was {}", config.query());
    //println!("The File provided was {}", config.filename());
    run(config)?;

    Ok(())
	
}

