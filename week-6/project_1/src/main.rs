fn main() {
    
    println!("Menu                  Price");
    println!("P = Poundo Yam/Edinkaiko Soup      - N3,200");
    println!("F = Fried Rice & Chicken           - N3,000");
    println!("A = Amala & Ewedu Soup             - N2,500");
    println!("E = Eba & Egusi Soup               - N2,000");
    println!("W = White Rice & Stew              - N2,500");

    
    let poundo_price = 3200;
    let fried_rice_price = 3000;
    let amala_price = 2500;
    let eba_price = 2000;
    let white_rice_price = 2500;

    
    let mut food_type = String::new();
    println!("\nEnter the type of food (P, F, A, E, W): ");
    std::io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");
    let food_type = food_type.trim(); 

    
    let mut quantity_str = String::new();
    println!("Enter the quantity: ");
    std::io::stdin()
        .read_line(&mut quantity_str)
        .expect("Failed to read input");
    let quantity: u32 = quantity_str.trim().parse().expect("Invalid quantity");

    
    let price = if food_type == "P" {
        poundo_price
    } else if food_type == "F" {
        fried_rice_price
    } else if food_type == "A" {
        amala_price
    } else if food_type == "E" {
        eba_price
    } else if food_type == "W" {
        white_rice_price
    } else {
        println!("Invalid food type!");
        return;
    };

    
    let mut total = price * quantity;

    
    if total > 10_000 {
        total = (total as f32 * 0.95) as u32; 
        println!("A 5% discount has been applied!");
    }

    
    println!(
        "Your total for {} food item(s) is: N{}",
        quantity,
        total
    );
    println!("Thank you for your order!");
}
