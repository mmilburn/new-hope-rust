mod reduce;
mod verify;
mod ntt;

fn main() {
    println!("{}", reduce::montgomery_reduce(1073491969));
}
