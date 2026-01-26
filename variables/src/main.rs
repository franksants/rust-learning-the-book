use std::io;

fn main() {
    let a = [[1, 2, 3, 4, 5], [5, 4, 3, 2, 1]];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[1][index];

    println!("The value of the element at index {index} is: {element}");
}
