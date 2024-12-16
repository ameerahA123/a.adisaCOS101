use std::io;

fn main(){
    println!("-----------------------STAFF LEVEL VALIDATION SYSTEM-----------------------------------");
    let aps12 = vec!["Intern","-","Paralegal","Placement"];
    let aps35 = vec!["Administrator","Research Assistant","Junior Associate","Classroom Teacher"];
    let aps58 = vec!["Senior Administrator","PhD Candidate","Associate","Snr Teacher"];
    let el1810 = vec!["Office Manager","Post-Doc Researcher","Senior Associate 1-2","Leadin Teacher"];
    let el21013 = vec!["Director","Senior Lecturer","Senior Associate 3-4","Deputy Principal"];
    let ses = vec!["CEO","Dean","Partner","Principal"];

    let mut input1 = String::new();
    let mut input2 = String::new();
   
    println!("What type of lawyer are you?");
    io::stdin().read_line(&mut input1).expect("Unable to read input");
    let mut lawyer = input1.trim();

    println!("Enter your years of experience (e.g 5-8)");
    io::stdin().read_line(&mut input2).expect("Unable to read input");
    let mut year = input2.trim();

    if lawyer == aps12[2]{
        if year == "1-2"{
            println!("You hold position APS 1-2");
        }
    }else if lawyer == aps58[2]{
        if year == "5-8"{
            println!("You hold position APS 5-8");
        }
    }else if lawyer == el1810[2]{
        if year == "8-10"{
            println!("You hold position EL1 8-10");
        }
    }else if lawyer == el21013[2]{
        if year == "10-13"{
            println!("You hold position EL2 10-13");
        }
    }else if lawyer == aps35[2]{
        if year == "3-5"{
            println!("You hold position APS 3-5");
        }
    }else if lawyer == ses[2]{
            println!("You hold position SES");
        
    }else{
        println!("Invalid input");
    }

}