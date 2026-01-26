use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Deu erro, meu amigo kkkk");

    let input: u32 = input
        .trim()
        .parse()
        .expect("Failed to convert the value to u32");

    if input > 10 {
        println!("{}", input);
    } else {
        println!("Error");
    }
}
