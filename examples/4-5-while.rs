
fn main() {
    // fizz buzz
    let mut num =1;
    while num < 101 {
     if num %15 == 0 {
         println!("fizz buzz")
     }
     else if num % 3 == 0 {
         println!("fizz")
     } else if num % 5 == 0 {
         println!("buzz");
     } else {
         println!("{}", num)
     }
     num += 1;
    }
 }