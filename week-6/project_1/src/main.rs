use std::io;

fn area(a: i32,b: i32,h:i32) {
   let area = h /2 * (a + b);
   println!("Area of trapezium is:{}",area);
}
fn area_2(a: f64,b: f64) {
   let area_2 = 0.5 * (a*b);
   println!("Area of rhombus is:{}",area_2);
}
fn area_3(a: i32,b:i32) {
   let area_3= b * a;
   println!("Area of parallelogram is:{}",area_3 );
}
fn area_4(b: i32){
   let area_4 = 6 *(b)^2;
   println!("Area of cube is: {}",area_4 );
}
fn volume(a: f64,b: f64){
   let volume=3.14159 * b * b * a;
   println!("Volume of Cylinder is: {}",volume);
}
fn main() {

     

   let mut inputf = String::new();
   println!("Enter input for calculation you want:");
   io::stdin().read_line(&mut inputf).expect("Failed to read input");
   let f:i32 = inputf.trim().parse().expect("Invalid input");

   if f == 1 {
   let mut input1 = String::new();
   println!("Enter input for parameter A:");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let a: i32 = input1.trim().parse().expect("Invalid input");

   let mut input2 = String::new();
   println!("Enter input for parameter B:");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let b: i32 = input2.trim().parse().expect("Invalid input");

   let mut input3 = String::new();
   println!("Enter input for parameter H:");
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   let h: i32 = input3.trim().parse().expect("Invalid input");
   area(a, b, h);
   }
   
   else if f==2 {
   let mut input1 = String::new();
   println!("Enter input for parameter A:");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let a: f64 = input1.trim().parse().expect("Invalid input");

   let mut input2 = String::new();
   println!("Enter input for parameter B:");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let b: f64 = input2.trim().parse().expect("Invalid input");

   area_2(a,b,/* i32*/);

   }
   
   else if f==3{
    let mut input1 = String::new();
   println!("Enter input for parameter A:");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let a: i32 = input1.trim().parse().expect("Invalid input");

   let mut input2 = String::new();
   println!("Enter input for parameter B:");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let b: i32 = input2.trim().parse().expect("Invalid input");

   area_3(a,b,/* i32*/);
   }

   else if f==4{
     let mut input1 = String::new();
   println!("Enter input for parameter B:");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let b: i32 = input1.trim().parse().expect("Invalid input");

   }

else if f==5{
   let mut input1 = String::new();
   println!("Enter input for parameter A:");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let a: f64 = input1.trim().parse().expect("Invalid input");

   let mut input2 = String::new();
   println!("Enter input for parameter B:");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let b: f64= input2.trim().parse().expect("Invalid input");
   
   volume(a,b, /* i32*/ )

}

}

