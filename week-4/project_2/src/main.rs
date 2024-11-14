use std::io;

fn main() {
    println!("Enter your age");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let age:u32 = input1.trim().parse().expect("Failed to input");

    

    println!("Are you experienced? (yes / no)");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let experience = input2.trim().to_lowercase();

    if age >= 40 {
        if experience == "yes" {
            println!("Your annual incentive is N1,560,000");
        }
    }

    if age >= 30 {
        if age < 40{
           if experience == "yes" {
            println!("Your annual incentive is N1,480,000"); 
           }
        }
    
    }

    if age < 28 {
           if experience == "yes" {
            println!("Your incentive is N1,300,000 per month"); 
           }
    }

    if experience == "no" {
            println!("Your annual incentive is N100,000"); 
    }
    
}
