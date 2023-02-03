use std::io;

fn main() {
    let mut input1 = String::new();
     let mut input2 = String::new();
      let mut input3 = String::new();

println!("Finding the real roots, Enter a: ");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let a:f32 = input1.trim().parse().expect("Not a valid number");

println!("Enter value of b: ");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let b:f32 = input2.trim().parse().expect("Not a valid number");

println!("Enter your value of c: ");
io::stdin().read_line(&mut input3).expect("Not a valid string");
let c:f32 = input3.trim().parse().expect("Not a valid number");

let d:f32 =( b *b ) - (4.0 * a *c);
 if d > 0.0 {
     println!("There are exactly two distinct roots");
 }
if d < 0.0 
{println!("There is no real root");
}
if d ==0.0 
    {println!("There is exactly one root");
}

let f = d.sqrt();
let first:f32 =((-1.0 * b) - f ) / (2.0 * a);
let second:f32=((-1.0 * b) + f) / (2.0 * a);

println!("The roots of the equation are: x = {} and x = {}",first,second );
}
