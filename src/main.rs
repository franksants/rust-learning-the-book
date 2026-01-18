use std::io;
fn main(){
    let mut guess = String::new();
    let secret_number = rand::random_range(1..100);

    println!("STARTING GUESSING GAME");
    println!("Guess a number(1 - 100): ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Digita a porra direito caralho!");
    println!("Guess number: {guess}");
    println!("Secret number: {secret_number}");
}
