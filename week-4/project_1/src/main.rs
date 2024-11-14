use std::io;

fn main() {
    println!("Enter the value of a:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read nput");
    let a:f64 = input1.trim().parse().expect("Failed to input number");
    println!(" a = {}" ,a);

    println!("Enter the value of b:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let b:f64 = input2.trim().parse().expect("Failed to input");
    println!("b = {}" ,b);

    println!("Enter the value of c:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f64 = input3.trim().parse().expect("failed to input number");
    println!("c = {}" ,c);

    let n:i32 = 2; 

    let d = b.powi(n) - (4.0 * a * c);

    if d == 0.0 {
        let  x = -b / (2.0 * a);
        println!("The value of you discriminant is {} (equal to zero). Therefore, your quadratic equation has exactly one root." ,d);
        println!(" The real root of your equation is {}" ,x);

    }

    if d >= 0.0{
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);

        println!("The value of you disctriminat is {}. Therefore, your equation has two distinct roots" ,d);
        println!("Your first root is {}" ,x1);
        println!("Your second root is {}" ,x2);
    }

    if d <= 0.0 {
        println!("The discriminant is {} (negative). Therefore, you have no distinct roots." ,d)
    }

    
}
