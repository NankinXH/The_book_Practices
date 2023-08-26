use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let the_opened_file = File::open("test.txt");
    // println!("{:#?}", the_opened_file);
    let greeting_file = match the_opened_file {
        Ok(file) => file,
        // Err(error) => panic!("Problem open the file {:#?}",  error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem create a the file {:#?}", e),
            },
            other_error=> {
                panic!("Problem opening the file {:?}", other_error);
            }
        }
    };

    println!("{:#?}", greeting_file);
}
