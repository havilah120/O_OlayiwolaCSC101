use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

     println!("Experienced is 10 years");

    println!("Enter the age of employee:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter years of experience:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience:i32 = input2.trim().parse().expect("Not a valid number");

if experience >= 10 {
    if age >= 40 {
      let i:i32 = 1560000;
      println!("The incentive is: {}",i);
    }
    else if age <=30 && age <40 {
       let i:i32 = 1480000;
      println!("The incentive is: {}",i);
    }
    else if age <30 {
       let i:i32 = 1300000;
       println!("The incentive is: {}", i);
    }
}
  else if experience < 10{
     let i:i32= 100000;
     println!("The incentive is: {}",i);
 }

}
