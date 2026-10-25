
#[allow(unused)]
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = format!("{s1}, {s2}!");

    println!("{s3}");
    let letter = s2.chars().nth(3);
    match letter {
        Some(c) => println!("The 3rd letter in {s2} is {c}"),
        None => println!("There is no 3rd letter.")
    }
    
    for c in s3.chars() {
        println!("{c}");
    }
}
