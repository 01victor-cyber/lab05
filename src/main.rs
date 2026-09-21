// is_even
fn is_even(n: i32) -> bool {
    if n % 2 == 0 { true } else { false }
}

fn main() {
    println!("{}", is_even(4)); //output: true
    println!("{}", is_even(7)); //output: false
    println!("{}", is_prime(7)); //true
    println!("{}", is_prime(10)); //false
    println!("{}", is_prime(1)); //false
}


// is_prime
fn is_prime(n: u32) -> bool {
   if n < 2 {
      return false;
   }

   let mut i = 2;
   while i * i <= n {
       if n % i == 0 {
          return false;
        }
        i = i +1;
     }

       true

}
