pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase(); // query is now a String
    contents
        .lines()
        .filter(move |line| {
            // We have to pass a string slice to `contains`
            line.to_lowercase().contains(&query)
        })
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents).collect::<Vec<_>>());
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents).collect::<Vec<_>>());

    }
}