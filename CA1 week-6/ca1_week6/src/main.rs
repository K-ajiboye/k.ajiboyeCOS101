// Rust program that develops medical diagnostic
// history of patients

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();

    println!("");
   
   println!("Enter your name: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");

   println!("Enter your age: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let age:i32 = input2.trim().parse().expect("Not a valid number");

   println!("Enter your email: ");
   io::stdin().read_line(&mut input3).expect("Not a valid string");

   println!("Enter your phone number: ");
   io::stdin().read_line(&mut input4).expect("Not a valid string");
   let phone_number:i32 = input4.trim().parse().expect("Not a valid number");

   println!("Enter number of siblings: ");
   io::stdin().read_line(&mut input5).expect("Not a valid string");
   let siblings:i32 = input5.trim().parse().expect("Not a valid number");

   println!("Enter number of children: ");
   io::stdin().read_line(&mut input6).expect("Not a valid string");
   let children:i32 = input6.trim().parse().expect("Not a valid number");

   println!("Enter your medical diagnosis: ");
   io::stdin().read_line(&mut input7).expect("Not a valid string");

   println!("Enter your village of residence: ");
   io::stdin().read_line(&mut input8).expect("Not a valid string");

   let let_alzheimer = 1_200_000 - (1_200_000 * 20 / 100);
   let let_arrythmia = 550_000 - (550_000 * 5 / 100);
   let let_ckd = 1_500_000 - (1_500_000 * 15 / 100);
   let let_diabetes = 800_000 - (800_000 * 10 / 100);
   let let_arthritis = 450_000 - (450_000 * 10 / 100);

   if age > 50 && children > 4
   {
       println!("You have a 20% discount, \nThis is your total amount");
   }
    
   if age == 30 && siblings > 4
   {
       println!("You have a 5% discount, \nThis is your total amount");
   }

   if age > 40 && siblings > 3
   { 
       println!("You have a 15% discount, \nThis is your total amount");
   }
    
    if age > 28 && age < 45 && children < 4
   {
    println!("You have a 10% dicount, \nThis is your total amount");
   }
    
    if age > 58 && siblings > 5
   {
    println!("You have a 10% discount, \nThis is your total amount");
   }
}
   


