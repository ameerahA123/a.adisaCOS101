
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("-----------------------------------PAU SMIS----------------------------------");
    let student_name = vec!["Oluchi Mordi",
    "Adams Aliyu",
    "Shania Bolade",
    "Adekunle Gold",
    "Blanca Edemoh"];

    let matric_number = vec!["ACC10211111",
    "ECO10110101",
    "CSC10328828",
    "EE11020202",
    "MEE10202001"];

    let department = vec!["Accounting",
    "Economics",
    "Computer",
    "Electrical",
    "Mechanical"];

    let level = vec!["300",
    "100",
    "200",
    "200",
    "100"];

    let sn = "STUDENT NAME";
    let mn = "MATRIC. NUMBER";
    let dept = "DEPARTMENT";
    let level_1 = "LEVEL";

    println!("|{:?}|{:?}|{:?}|{:?}|" ,sn,mn,dept,level_1);

    let header = format!("|{:?}|{:?}|{:?}|{:?}|" ,sn,mn,dept,level_1);

    let mut file = std::fs::File::create("pau_smis.txt").expect("Create failed");

    file.write_all(header.as_bytes()).expect("Write failed");
    file.write_all(title.as_bytes()).expect("Write failed");
    

    for i in 0..student_name.len(){
        let content = format!("\n|{:?}|{:?}|{:?}|{:?}|\n",student_name[i],matric_number[i],department[i],level[i]);

        println!("{}",content);

        file.write_all(content.as_bytes()).expect("Write failed");
    }

    let header = format!("\n|{:?}|{:?}|{:?}|{:?}|\n" ,sn,mn,dept,level_1);
   
    
    

  

}
