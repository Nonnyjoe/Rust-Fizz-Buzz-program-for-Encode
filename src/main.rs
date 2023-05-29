fn main() {
    println!("Hello, world!");
    fizz_buzz(301);
}

fn fizz_buzz(number: u128) {
    let mut check: u128 = 1;
    let mut fizz: u128 = 0;
    let mut buzz: u128 = 0;
    let mut fizz_buzz: u128 = 0;

    while check <= number {
        if check % 3 == 0 && check % 5 == 0 {
            fizz_buzz += 1;
            println!("Fizz Buzz")
        } else if check % 3 == 0 {
            fizz += 1;
            println!("Fizz");
        } else if check % 5 == 0 {
            buzz += 1;
            println!("Buzz");
        } else {
            println!("{}", check)
        }
        check += 1;
    }
    println!("Number of Fizz: {}", fizz);
    println!("Number of buzz: {}", buzz);
    println!("Number of Fizz_buzz: {}", fizz_buzz);
}
