fn main() {
    let mut stop_condtition = 0;
    loop {
        println!("Try again!");
        if stop_condtition > 5 {
            break;
        }
        stop_condtition += 1;
    }
}
