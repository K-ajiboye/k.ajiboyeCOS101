use std::io;

fn main() {    
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Distance");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let d:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter Time");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f32 = input2.trim().parse().expect("Not a valid number");

    let mut spd:f32 = d / t;

    println!("Average speed: {}", spd);

}