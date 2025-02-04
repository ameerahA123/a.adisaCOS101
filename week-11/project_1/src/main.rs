use std::io::Read;
use std::io;

fn main() {



    println!("If you are 
                         An Admistrator enter '1'
                         A Project Manager enter '2'
                         An Employee enter '3'
                         A Customer enter '4'
                         A Vendor enter '5'" );

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let input: i32 = input1.trim().parse().expect("Not a valid input");


    if input==1{
        admin();
    }

    if input==2{
        pm();
    }

    if input==3{
        emp();
    }

    if input==4{
        cus();
    }

    if input==5{
        ven();
    }

}

fn admin(){
    let mut file = std::fs::File::open("globacom_db.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn pm(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}
fn emp(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}
fn cus(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}
fn ven(){
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}


