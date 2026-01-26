fn main() {
    let mut stop_conditition = 0;
    loop {
        println!("Try again!");
        if stop_conditition > 5 {
            break;
        }
        stop_conditition += 1;
    }
}
