use std::io;

fn main() {
    println!("CARBON FOOTPRINT CALCULATOR FOR PHONES AND LAPTOPS FOR MEMBERS OF GROUP 18");
    //Carbon Footprint = Activity level of device * Emmission Factor
    //to ascertain activity level;

    println!("How many group members are present in GROUP 18?");
    let mut group_members = String::new();
    io::stdin().read_line(&mut group_members).expect("Not a valid String");
    let group_members:u64 = group_members.trim().parse().expect("Not a valid Number");


    for x in 0..group_members{
        let mut device = String::new();
        println!("If you are using a laptop enter '1' -- If you are using a phone enter '2'");
        io::stdin().read_line(&mut device).expect("Not a valid String");
        let device:u64 = device.trim().parse().expect("Not a valid Number");

    
        let mut power_consumption = 0.0; 
        let mut emission_factor = 0.0;  
        let mut laptop_hours = 0.0;
        
        if device == 1 {
            println!("Enter your laptop model:");
            let mut laptop_model = String::new();
            io::stdin().read_line(&mut laptop_model).expect("Not a valid String");
            let laptop_model= laptop_model.trim();

            if laptop_model == "HP Spectre"{
                let mut power_consumption:f32 = 90.0;
                let mut emission_factor:f32 = 0.475; 

                let mut laptop_hours = String::new();
                println!("Enter hours of device usage:");
                io::stdin().read_line(&mut laptop_hours).expect("Not a valid String");
                let laptop_hours:f32 = laptop_hours.trim().parse().expect("Not a valid Number");

                let energy = (power_consumption / 1000.0) * laptop_hours;
                let daily_carbon_footprint = energy * emission_factor;
                let annual_carbon_footprint = daily_carbon_footprint * 365.0;

                println!("Your laptop model is the {}.",laptop_model );
                println!("You use your laptop for {} hours.",laptop_hours);
                println!("The power consumption of your laptop is {}W",power_consumption );
                println!("The emission factor of your laptop is {}KgCO2",emission_factor );
                println!("THE DAILY CARBON FOOT PRINT OF YOUR PHONE is {}kgCO₂e",daily_carbon_footprint);
                println!("THE ANNUAL CARBON FOOT PRINT OF YOUR PHONE is {}kgCO₂e",annual_carbon_footprint);



            }else if laptop_model == "HP Laptop 14"{
                let mut power_consumption:f32 = 45.0; 
                let mut emission_factor:f32 = 0.475;

                let mut laptop_hours = String::new();
                println!("Enter hours of device usage:");
                io::stdin().read_line(&mut laptop_hours).expect("Not a valid String");
                let laptop_hours:f32 = laptop_hours.trim().parse().expect("Not a valid Number");

                let energy = (power_consumption / 1000.0) * laptop_hours;
              
                let daily_carbon_footprint = energy * emission_factor;
                let annual_carbon_footprint = daily_carbon_footprint * 365.0;

                println!("Your laptop model is the {}.",laptop_model );
                println!("You use your laptop for {} hours.",laptop_hours);
                println!("The power consumption of your laptop is {}W",power_consumption );
                println!("The emission factor of your laptop is {}KgCO2",emission_factor );
                println!("THE DAILY CARBON FOOT PRINT OF YOUR PHONE is {}kgCO₂e",daily_carbon_footprint);
                println!("THE ANNUAL CARBON FOOT PRINT OF YOUR PHONE is {}kgCO₂e",annual_carbon_footprint);

            }


        }if device == 2{
            println!("Enter your phone model:");
            let mut phone_model = String::new();
            io::stdin().read_line(&mut phone_model).expect("Not a valid String");
            let phone_model = phone_model.trim();

            if phone_model == "Iphone 15 Pro Max"{
                let mut power_consumption:f32 = 8.0;
                let mut emission_factor:f32 = 0.5; 

                let mut phone_hours = String::new();
                println!("Enter hours of device usage:");
                io::stdin().read_line(&mut phone_hours).expect("Not a valid String");
                let phone_hours:f32 = phone_hours.trim().parse().expect("Not a valid Number");

                let energy = (power_consumption / 1000.0) * phone_hours;
                let daily_carbon_footprint = energy * emission_factor;
                let annual_carbon_footprint = daily_carbon_footprint * 365.0;

                println!("Your phone model is the {}.",phone_model );
                println!("You use your phone for {} hours.",phone_hours);
                println!("The power consumption of your phone is {}W",power_consumption );
                println!("The emission factor of your phone is {}KgCO2",emission_factor );
                println!("THE DAILY CARBON FOOT PRINT OF YOUR PHONE is {}kgCO₂e",daily_carbon_footprint);
                println!("THE ANNUAL CARBON FOOT PRINT OF YOUR PHONE is {}kgCO₂e",annual_carbon_footprint);



            }else if phone_model == "Redmi 14C"{
                let mut power_consumption:f32 = 6.0; 
                let mut emission_factor:f32 = 0.48;

                let mut phone_hours = String::new();
                println!("Enter hours of device usage:");
                io::stdin().read_line(&mut phone_hours).expect("Not a valid String");
                let phone_hours:f32 = phone_hours.trim().parse().expect("Not a valid Number");

                let energy = (power_consumption / 1000.0) * phone_hours;
                
                let daily_carbon_footprint = energy * emission_factor;
                let annual_carbon_footprint = daily_carbon_footprint * 365.0;

                println!("Your phone model is the {}.",phone_model );
                println!("You use your phone for {} hours.",phone_hours);
                println!("The power consumption of your phone is {}W",power_consumption );
                println!("The emission factor of your phone is {}KgCO2",emission_factor );
                println!("THE DAILY CARBON FOOT PRINT OF YOUR PHONE is {}kgCO₂e",daily_carbon_footprint);
                println!("THE ANNUAL CARBON FOOT PRINT OF YOUR PHONE is {}kgCO₂e",annual_carbon_footprint);

                
            }

        }
    }    

}