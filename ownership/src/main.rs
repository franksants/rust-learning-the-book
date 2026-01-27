fn main() {
    let s = 200;
    println!("{s}");
    {
        let s = "hello world";
        println!("{}", s);
    }
    let s = 50;
    println!("{s}");
}
