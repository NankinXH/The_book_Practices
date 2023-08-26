use std::fs::File;
use std::io::{self, Read};
use std::error::Error;

fn read_username (path: &str)->Result<String, io::Error> {

   let username_file = File::open(path);
   let mut username_result = match username_file {
        Ok(file) => file,
        Err(error) => return Err(error)
   };

   let mut username = String::new();
   match username_result.read_to_string(& mut username) {
    Ok(_)=> Ok(username),
    Err(e) => Err(e),
   }
   
}
fn read_username_from_file()->Result<String, io::Error> {
    let mut username_string = String::new();
    // We could even shorten these code by chaining method calls immediately after the ? operator.
    File::open("username.txt")?.read_to_string(&mut username_string)?;
    Ok(username_string)
    
}

fn last_char_of_first_line(text: &str) ->Option<char> {
    text.lines().next()?.chars().last()
}

fn main()-> Result<(), Box<dyn Error>> {
    // let greeting_file =
    //     File::open("test.txt").expect("test.txt should be included in this project");
    let username = read_username("username.txt");
    let username_2 = read_username_from_file();
    // The ? operator can only be used in a fuction that returns Result or Option.
    let greeting_file = File::open("ee.tsx")?;
    let last_char = last_char_of_first_line("last char of first line");
    println!("{:?}, {:?},{:?} {:?}", username, username_2, greeting_file, last_char);
    Ok(())
    
}
