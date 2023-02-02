 use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    println!("Welcome to Globacom Database \n Insert the number for the appropriate placeholder");
    println!(" 1-Administrator \n 2-Project Manager \n 3-Employee\n 4-Customer \n 5-Vendor");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input:i32 = input.trim().parse().expect("Invalid input");
    
    if input == 1
    {
        code_1();
    }
    else if input == 2
    {
        code_2();
    }
    else if input == 3
    {
        code_3();
    }
    else if input == 4
    {
        code_4();
    }
    else if input == 5
    {
        code_5();
    }
    else{
        println!("This is an invalid input");
    }
}

fn code_1() {
    let mut file = File::open("globacom_dbase.sql").expect("Can't open database!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("sorry,Can't read the database");
    print!("Here is your Database \n{}",contents);
    }
    
fn code_2(){
    let mut file = File::open("project_tb.sql").expect("Can't open database!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("sorry,Can't read the database");
    println!("Here is your Database \n{}",contents)
}
fn code_3(){
    let mut file = File::open("staff_tb.sql").expect("Can't open database!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("sorry,Can't read the database");
    println!("Here is your Database \n{}",contents)
}
fn code_4(){
    let mut file = File::open("customer_tb.sql").expect("Can't open database!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("sorry,Can't read the database");
    println!("Here is your Database \n{}",contents)
}
fn code_5(){
    let mut file = File::open("dataplan_tb.sql").expect("Can't open database!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("sorry,Can't read the database");
    println!("Here is your Database \n{}",contents)
}