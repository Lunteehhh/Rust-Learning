use std::fs::File;
use std::io::ErrorKind;

#[allow(unused)]
fn main() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(file) => file ,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem with creating file: {e:?}"),
            }
            _ => {
                panic!("Problem openng the file: {error:?}");
            }
        }
    };

    let file = File::open("no_file.xtx")
        .expect("no_file.xtx doesn't exist");
}
