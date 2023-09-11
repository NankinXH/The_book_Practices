#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32,
}
fn main() {
    let mut rect_list = vec![
        Rectangle {
            height: 22,
            width: 11,
        },
        Rectangle {
            height: 88,
            width: 66,
        },
        Rectangle {
            height: 94,
            width: 23,
        },
    ];
    let mut sort_operation: Vec<String> = vec![];
    let mut num_sort_operation = 0;
    let value = String::from("by key operator");
    rect_list.sort_by_key(|r| r.width);
    println!("{:#?}", rect_list);
    rect_list.sort_by_key(|r| {
        // sort_operation.push(value);
        num_sort_operation+=1;
        r.height
    });
    println!("{:#?}, times: {}", rect_list, num_sort_operation);
}
