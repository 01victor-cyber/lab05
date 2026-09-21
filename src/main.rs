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
    let number = 12345;
    let result = digit_sum(number);
    println!("The sum of digits of {} is {}", number, result);
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

// Implementation of digit_sum funcation
fn digit_sum(n: i32) -> i32 {
    let mut sum = 0;
    let mut temp = n;

    while temp != 0 {
       let last_digit = temp % 10;
       sum += last_digit;
       temp /= 10;
}

    sum
}
