fn main() {
    println!("Hello, world!");
    fizz_buzz(4);
}

fn fizz_buzz(number: u128) {
    if number > 5 {
        println!("above");
    } else {
        println!("below");
    }
}
