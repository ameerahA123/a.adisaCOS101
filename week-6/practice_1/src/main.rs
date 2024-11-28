fn main() {
    let name = "Ameerah Adisa";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 57 Lekki-Epe Expressway, Ibeju-Lekki, Lagos.";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}",uni,addr);

    let department:&'static str = "Compute Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {} \nSchool: {}",department, school);
}
