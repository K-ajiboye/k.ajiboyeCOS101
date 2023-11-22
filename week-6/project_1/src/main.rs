
use std::io;

fn main() {

 let mut count = 0;

 loop {
   count += 1;
   if count == 150
   {
            break;
   }    
  let mut input1 = String::new();
  let mut input2 = String::new();
  let mut input3 = String::new();

   println!("Enter '1' if your answer is yes.\nEnter '0' if your answer is no"); 

   println!("Are you a current class Rep?: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let input1:i64 = input1.trim().parse().expect("Not a valid number");
 
   println!("Are you not in 100 Level?: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let input2:i64 = input2.trim().parse().expect("Not a valid number");

   println!("Is your cgpa above 4.0?: ");
   io::stdin().read_line(&mut input3).expect("Not a valid string");
   let input3:i64 = input3.trim().parse().expect("Not a valid number"); 

   if input1 == 1 && input2 == 1 && input3 == 1 {

   println!("What's your name? : ");
   let mut name = String::new();
   io::stdin().read_line(&mut name).expect("Failed to read input");

   println!("What's your email? : ");
   let mut email = String::new();
   io::stdin().read_line(&mut email).expect("Failed to read input");

   println!("What department are you? : ");
   let mut department = String::new();
   io::stdin().read_line(&mut department).expect("Failed to read input");

   println!("What's your state of origin? : ");
   let mut state = String::new();
   io::stdin().read_line(&mut state).expect("Failed to read input");

   println!("You can vote");
   }

   else {
       println!("Sorry, you are not eligible to vote");
    }
  }
}