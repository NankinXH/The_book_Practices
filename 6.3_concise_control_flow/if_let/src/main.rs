use rand::Rng;
fn main() {
    let rand_number = rand::thread_rng().gen_range(1..4);
    let config_max = Some(rand_number);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}!", max),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}!", max);
    } else {
        println!("11112222");
    }
}
