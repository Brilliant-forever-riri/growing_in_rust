fn add(x: u64, y:u64)-> u64{
   return x+y // This line adds x and y , and the result is returned
}
fn multiply(h: u128, m: u128)-> u128{
   
   return h*m  // This line multiplies, and the result is returned
} 
fn main(){
    let result = add(234567892345634, 987563345450234);
    let result_2 = multiply(2234566848745893836, 238463939);
    println!("the sum is {result}");
    println!("the product is {result_2}");
}
