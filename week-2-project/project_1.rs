fn main() {
    let p = 520_000_00;
    let r = 10;
    let n = 5;
    let a = p * ((1 + (r / 100))) ^ n;
    println!("Amount is {}", a);
    let ci = a - p;
    println!("Compund interest is {}", ci);
}