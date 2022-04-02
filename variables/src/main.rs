fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {} seconds", THREE_HOURS_IN_SECONDS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    shadowing();
}
 fn shadowing() {
     println!("SHADOWING CONTEXT");

     let spaces = "    ";
     let spaces = spaces.len();
     println!("The value of scapes: {}", spaces);

     let x = 5;
     let x = x + 1;
     {
         let x = x * 2;
         println!("The value of x in the inner scope is: {}", x);
     }

     println!("The value of x is: {}", x);
 }