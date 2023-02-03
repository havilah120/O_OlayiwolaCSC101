fn main() {
    //Name Vector
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    //Age Vector
    let age= vec![16,17,19,20,21,18,23];

    print!("Age allocation");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {
         //iterating through i on the vector
        print!("{} is {} years old",name[i],age[i]);
    }
}
