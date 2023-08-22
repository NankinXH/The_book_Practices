use rand::Rng;

fn roll_dice() -> i32 {
    rand::thread_rng().gen_range(6..8)
}
fn main() {
    let roll_count = 0;
    dice_roll(roll_count);
}
// The method is used to roll a dice by random.
fn dice_roll(mut roll_count: i32) -> String {
    // I define the counter to set down the number of I dicing;
    let dice_num = roll_dice();
    roll_count += 1;
    println!("we have rolled {} times already!", roll_count);
    match dice_num {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(roll_count),
    }
}

fn add_fancy_hat() -> String {
    let action_string = String::from("You roll a action adding a fancy hat!");
    println!("{:?}", action_string);
    action_string
}
fn remove_fancy_hat() -> String {
    let action_string = String::from("You roll a action moving a fancy hat!");
    println!("{:?}", action_string);
    action_string
}
// If you roll a result neither 3 or 7, you will get a chance to roll.
fn reroll(roll_count: i32) -> String {
    let action_string = String::from("You have got a chance to reroll the dice!");
    println!("{:?}", action_string);
    dice_roll(roll_count)
}
