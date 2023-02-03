use std::io;

fn main() {

//P
let pricep:f64=3200.0;
let pricef:f64=3000.0;
let pricea:f64= 2500.0;
let pricee:f64 = 2000.0;
let pricew:f64 = 2500.0;

println!("(S/N)   Menu                           Price ");
println!("P(1)       Poundo Yam/Edinkaiko Soup      3200  ");
println!("F(2)      Fried Rice & Chicken           3000  ");
println!("A(3)       Amala & Ewedu Soup             2500  ");
println!("E(4)      Eba & Egusi Soup               2000  ");
println!("W(5)      White Rice & Stew              2500  ");

let mut input1 = String::new();
let mut input2=  String::new();

println!("Enter food type:");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let foodtype:f64= input1.trim().parse().expect("Not a valid (S/N)");

println!("Enter quantity of food:");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let quantity:f64= input2.trim().parse().expect("Not a valid number");

if foodtype == 1.0{
   let total:f64= quantity * pricep;
   println!("Total is {}",total );
   if total > 10000.0 {
       let discount:f64 = total - (0.05 * total);
   }
   else {
