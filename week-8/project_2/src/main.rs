fn main() {
    println!("--------------------------ERNEST & YOUNG GLOBAL LIMITED---------------------------");
    let developers = vec![("Ada",4),("Alice",5),("Emeka",6)];

    let mut top_candidate = developers[0];

    for developer in developers{
        if developer.1 > top_candidate.1{
           top_candidate = developer;
           
        }
    } 

    println!("The person with the most experience is {:?} with {:?} years of experience.", top_candidate.0, top_candidate.1);

}
