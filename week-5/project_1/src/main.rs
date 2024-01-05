// Rust program to calculate the roots of a quadratic equation

use std::io;

fn main(){
    println!("Enter the values for 'a' : ", );
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the values for 'b' : ", );
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter values for 'c' : ",);
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    let d:f64 = b.powf(2.0) - 4.0*a*c;

    if d > 0.0
    {
        println!("\nThe equation has two distinct roots");
    }

    else if d < 0.0
    {
        println!("\nThe equation has no real roots");
    }
    
    else if d == 0.0 {
        println!("\nThe equation has only one real root");
    }
    let roots1:f64 = (-b + d.powf(0.5)) / 2.0;
    let roots2:f64 = (-b - d.powf(0.5)/ 2.0;

    println!("\nThe roots of the equation are {} and {}",roots1 ,roots2 );




}