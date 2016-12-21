// Make me compile!

fn something() -> Result<i32, std::num::ParseIntError> {
    "3".parse::<i32>().map(|i| i * 4)
}

fn main() {
    match something() {
        Ok(..) => println!("You win!"),
        Err(e) => println!("Oh no something went wrong: {}", e),
    }
}
