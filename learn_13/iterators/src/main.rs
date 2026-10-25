fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    let v3: Vec<i32> = (1..=100).collect();

    let v3_squared: Vec<i32> = v3.iter().map(|x| x.pow(2)).collect();

    let v3_filter: Vec<i32> = v3_squared.iter().filter(
        |x| *x % 2 == 0
    ).copied().collect();

    let v3_filter2: Vec<i32> = v3.iter().filter(
        |x| *x * *x % 2 == 0
    ).copied().collect();

    println!("{v3_filter2:#?}")
}

