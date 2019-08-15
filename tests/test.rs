#[cfg(test)]
mod tests{
	use minigrep::*;
	#[test]
	fn one_result() {
        	let query = "duct";
        	let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        	assert_eq!(
			vec!["safe, fast, productive."],
            		search(query, contents)
        	);
    	}

	#[test]
	fn one_result_case_insensitive() {
        	let query = "Duct";
        	let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        	assert_eq!(
			vec!["safe, fast, productive."],
            		search_insensitive(query, contents)
        	);
    	}

}
