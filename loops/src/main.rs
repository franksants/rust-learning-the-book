fn main() {
    //let mut stop_conditition = 0;
    //loop {
    //    println!("Try again!");
    //    if stop_conditition > 5 {
    //        break;
    //    }
    //    stop_conditition += 1;
    //}

    //let mut counter = 50;
    //let result = loop {
    //    counter -= 10;

    //it will become 50 back hahaha
    //it is only a crazy code, let's continue!
    //    if counter < 40 {
    //        break counter + 20;
    //    }
    //};
    //println!("{}", result);

    let mut counter: f64 = 0.0;
    let mut counter_loop: u32 = 0;
    while counter < 50.0 {
        println!("{}", counter);
        counter += 0.5;
        counter_loop += 1;
    }
    println!("Final count: {}", counter_loop);
}
