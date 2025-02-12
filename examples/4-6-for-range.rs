
fn main() {
    // 优先使用 for range over loop 和 while
   for i in 0..5 {
    print!("{}\t", i);
   }
   println!("");

   for i in 0..=5 {
    print!("{}\t", i);
   }
   println!("");

   let myarray = [1, 2, 3];
   for i in myarray.iter() {
    print!("{:?}\t", i);
   }
   println!("");

   let mut myarray = [1, 2, 3];
   for i in myarray.iter_mut() {
    *i *=  2;
   }
   for i in myarray.iter() {
    print!("{:?}\t", i);
   }
   println!("");
}