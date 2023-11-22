  
use std::io;

fn main() {
    for x in 1..501{
    println!("Researcher Incentive Calculator System");
    let mut input1 = String::new();
    
    println!("\nEnter name of Researcher: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Not a valid input");

    println!("\nEnter no of papers published: ");
    io::stdin().read_line(&mut input1).expect("Not a valid number");
    let no_of_papers_published:i32 = input1.trim().parse().expect("Not a valid number");

    if no_of_papers_published ==3 && no_of_papers >= 3 && no_of_papers == 5 && no_of_papers <= 5 {
        let incentive:f32 = 500_000.00;
        println!("You,{} have an incentive of: N{}",name,incentive);

    } if no_of_papers_published >== 5 && no_of_papers_published <= 10{
         let incentive:f32 = 800_000.00;
         println!("You,{} have an incentive of: N{}",name,incentive);

    } if no_of_papers_published == 10 && no_of_papers_published >= 10{
        let incentive:f32 = 1_000_000.00;
        println!("You,{} have an incentive of: N{}",name,incentive);

    } if no_of_papers_published <= 3{
         let incentive:f32 = 100_000.00;
         println!("You,{} have an incentive of: N{}",name,incentive);

     println!("count {} ", x);
     println!(" ");//just an empty line               
     }
    }   