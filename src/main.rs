mod reduce;
mod verify;

fn main() {
    println!("{}", reduce::montgomery_reduce(1073491969));
}
