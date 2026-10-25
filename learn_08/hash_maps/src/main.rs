use std::collections::HashMap;

fn word_counter(text: &str) {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }
    println!("{map:?}");
}

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("Orange");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("team: {team_name} score: {score}");
    
    scores.insert(String::from("blue"), 30);
    for (key, val) in &scores {
        println!("{key}: {val}");
    }

    scores.entry(String::from("green")).or_insert(10);
    scores.entry(String::from("yellow")).or_insert(10);
    println!("{scores:?}");

    word_counter("hallo welt welt hallo test wort test welt hallo neu");
}
