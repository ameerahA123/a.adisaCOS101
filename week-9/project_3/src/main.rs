use std::io::Write;


fn main() {
    let noc = "NAME OF COMMISSIONER";
    let min = "MINISTRY";
    let gz = "GEOPOLITICAL ZONE";


    let names = vec!["Aigbogun Alamba Dauda",
                     "Murtala Afeez Bendu ",
                     "Okorocha Calistus Ogbona",
                     "Adewale Jimoh Akanbi",
                     "Osazuwa Faith Etieye"];

    let mins = vec!["Internal Affairs",
                    "Justice",
                    "Defense",
                    "Power & Steel",
                    "Petroleum"];


    let geo = vec!["South West",
                   "North East",
                   "South South",
                   "South West",
                   "South East"];


    
    println!("| {} | {} | {} |",noc,min,gz);
    let noc_1 = format!("|{}|{}|{}|",noc,min,gz);

    let mut file = std::fs::File::create("CONVICTED_MINISTERS.txt").expect("create failed");
    file.write_all(noc_1.as_bytes()).expect("Write failed");

   
    

    for i in 0..names.len(){
        println!("\n|{:?}|{:?}|{:?}|\n",names[i],mins[i],geo[i]);
        let content = format!("\n|{:?}|{:?}|{:?}|\n",names[i],mins[i],geo[i]);
        file.write_all(content.as_bytes()).expect("Write failed");
        
    }


    
}
