use std::io::Write;

fn main() {


    let mut larger ="                              33 Export\n
                    Desperados\n
                    Goldberg\n
                    Guilder\n
                    Heineken\n
                    Star\n


                        

                        ";

    let mut stout ="                                          Legend\n
                    Turbo King\n
                    Williams\n


                      

                      ";

    let mut non_alcoholic ="                                           Maltina\n
                            Amstel Malta\n
                            Malta Gold\n
                            Fayrouz\n



                              ";
    
    let mut drinks = std::fs::File::create("Nigeria_breweries_plc.txt").expect("create failed");
    
    drinks.write_all("LARGER\n".as_bytes()).expect("Write failed.");

    drinks.write_all(larger.as_bytes()).expect("Write failed.");

    drinks.write_all("STOUT\n".as_bytes()).expect("Write failed.");

    drinks.write_all(stout.as_bytes()).expect("Write failed.");

    drinks.write_all("NON-ALCOHOLIC\n".as_bytes()).expect("Write failed.");

    drinks.write_all(non_alcoholic.as_bytes()).expect("Write failed.");

    println!("Data successfully written to file!");

}
