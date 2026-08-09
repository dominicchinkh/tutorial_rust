pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    // The functional programming style prefers to minimize the amount of mutable state 
    // to make code clearer

    /*
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
            
        results
    */

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {

    /*
        // Note that query is now a String rather than a string slice because calling to_lowercase
        // creates new data rather than referencing existing data
        let query = query.to_lowercase();

        let mut results = Vec::new();

        for line in contents.lines() {
            
            // When we pass query as an argument to the contains method now, we need to add an 
            // ampersand because the signature of contains is defined to take a string slice
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    */

    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            search_case_insensitive(query, contents)
        );
    }
}