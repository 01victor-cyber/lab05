// is_even
fn is_even(n: i32) -> bool {
    if n % 2 == 0 { true } else { false }
}

fn main() {
    println!("{}", is_even(4)); //output: true
    println!("{}", is_even(7)); //output: false
}
