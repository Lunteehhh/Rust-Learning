mod enums;

use enums::SpreadSheet;

fn main() {
    // immutable Vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element ist {third}");

    let third: Option<&i32> = v.get(10);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("there is no third element."),
    }

    for i in &v {
        println!("{i}");
    }


    // mutable Vector
    let mut v2 = vec![1, 2, 3, 4, 5];

    for i in &mut v2 {
        *i *= *i;
    }

    for i in &v2 {
        println!("{i}");
    }


    // Multiple type Vector
    let row = vec![
        SpreadSheet::Float(3.145),
        SpreadSheet::Text(String::from("blue")),
        SpreadSheet::Int(42),
    ];
    
    for val in &row {
        match val {
            SpreadSheet::Int(x) => println!("Integer: {x}"),
            SpreadSheet::Float(x) => println!("Float: {x}"),
            SpreadSheet::Text(x) => println!("Text: {x}")
            
        }
    }
}
