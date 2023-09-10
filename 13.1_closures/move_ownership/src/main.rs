use std::thread;

fn main() {
    let immutable_vec = vec![1, 23, 43];
    println!("Before defining the closure");
    let only_borrows = || println!("From closure: {:?}", immutable_vec);
    println!("Before calling the closure");
    only_borrows();
    println!("After calling the closure");

    let mut mut_vec = vec![32, 321, 0];
    println!("Before defining the closure");
    let mut mutable_borrows = || mut_vec.push(7);
    println!("Before calling the closure");
    mutable_borrows();
    println!("After calling the closure: {:?}", mut_vec);

    let mut thread_vec = vec![32, 43, 76];
    thread::spawn(move || {
        thread_vec.push(7777);
        println!("From thread: {:?}", thread_vec);
    })
    .join()
    .unwrap();
    // Test the github services!
}
