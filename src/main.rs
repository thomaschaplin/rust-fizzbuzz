fn fizzbuzz(count: i32) -> () {
    for i in 1..=count {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}

fn main() {
    fizzbuzz(100);
}
