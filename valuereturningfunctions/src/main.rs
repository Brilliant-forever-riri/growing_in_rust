fn add(x: u64, y:u64)-> u64{
    x+y // This line adds a and b, and the result is returned
}
fn multiply(h: u128, m: u128)-> u128{
   
    h*m  //This line multiplies h and m, and the result is returned 
} 
fn main(){
    let sum = add(23456789234, 987563450234); //this line calls the function using the given numbers 
    let product = multiply(2234566848745893836, 238463939); //this line calls the function using the given numbers 
    println!("the sum is {sum}"); //this function prints the result
    println!("the product is {product}"); //this function prints the result
}