// fn main() {
//    let x = 5; 
//    println!("The value of x is {x}");
//    x = 6;
//    println!("The valus of x is {x}");
// }
// The above code will throw a error bcoz let x is immutable; 
// you need to use 'let mut x' to make it mutable

// fn main(){
//     let mut x = 5;
//     print!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main(){
    let x = 5;  
     let x = x + 1;   
    {
      let x = x * 2;  // but here we required to use let x
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
// output ->
// The value of x in the inner scope is: 12
// The value of x is: 6

// if we don't use let in the shadowing/inner scope the outer x will also get's its value.
// let x create a new variable inside the scope which value can only be used inside it.



