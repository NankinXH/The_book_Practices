use std::vec;

fn main() {
    let num_list_1: Vec<i32> = vec![17, 1, 33, 77, 10];

    let num_list_2: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    largest_num(&num_list_1);
    largest_num(&num_list_2);

    println!("The maximum number is: {:?}", largest(&num_list_2));
    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    

    largest_char(&char_list);


}

fn largest_num(num_list: &[i32]) -> &i32 {
    let mut largest = &num_list[0];
    for cur_num in num_list {
        if cur_num > &largest {
            largest = cur_num;
        }
    }
    println!("the largest number in the vector is: {}", largest);
    largest
}

fn largest_char(char_list: &[char]) -> &char {
    let mut largest_char = &char_list[0];
    for cur_char in char_list {
        if cur_char > &largest_char {
            largest_char = cur_char;
        }
    }
    println!("the largest char in the vector is: {}", largest_char);
    largest_char
}

// #[derive(Debug)]
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest_one = &list[0];
    for cur in list {
        if cur > &largest_one {
            largest_one = cur;
        }
    }
    // println!("The largest one is the :{:#?}", largest_one);
    largest_one
}
