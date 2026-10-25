pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensetive<'a>(
    query: &str, 
    content: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensetive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive
Pick three.
Duck tape.";

        assert_eq!(
            vec!["safe, fast, productive"],
            search(query, content)
        );
    }

    #[test]
    fn case_insensetive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensetive(query, content)
        );
    }
}
