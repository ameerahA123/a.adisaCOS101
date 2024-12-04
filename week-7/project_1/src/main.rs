use std::io;


fn main() {
    println!("-------------------------------MY CALCULATOR--------------------------------- ");
    println!("Area of trapezium = 1");
    println!("Area of rhombus = 2");
    println!("Area of parallelogram = 3");
    println!("Area of cube = 4");
    println!("Volume of cylinder = 5");

    println!("Input your options (1, 2, 3, 4, 5): ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Unable to read input");
    let t:u32 = input1.trim().parse().expect("Failed to input");


    if t == 1 {
        trapezium();
    }else if t == 2{
        rhombus();

    }else if t == 3{
        parallelogram();

    }else if t == 4{
        cube();

    }else if t == 5{
        volume();

    }else {
        println!("Invalid input!");
    }

}
fn trapezium(){
    println!("Input height:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Unable to read input");
    let h:f32 = input2.trim().parse().expect("Unable to input");

    println!("Input base1:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Unable to read input");
    let b_1:f32 = input3.trim().parse().expect("Unable to input");

    println!("Input base2:");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Unable to read input");
    let b_2:f32 = input4.trim().parse().expect("Unable to input");

    let area_trapezium:f32 = (h / 2.0)*(b_1 + b_2);

    println!("The area of the trapezium is {}" ,area_trapezium);

}

fn rhombus(){
    println!("Input first diagonal:");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("Unable to read input");
    let d_1:f32 = input5.trim().parse().expect("Unable to input");

    println!("Input second diagonal:");
    let mut input6 = String::new();
    io::stdin().read_line(&mut input6).expect("Unable to read input");
    let d_2:f32 = input6.trim().parse().expect("Unable to input");

    let area_rhombus:f32 = (0.5)*(d_1 * d_2);

    println!("The area of the rhombus is {}" ,area_rhombus);
}

fn parallelogram(){
    println!("Input base:");
    let mut input7 = String::new();
    io::stdin().read_line(&mut input7).expect("Unable to read input");
    let b:f32 = input7.trim().parse().expect("Unable to input");

    println!("Input altitude:");
    let mut input8 = String::new();
    io::stdin().read_line(&mut input8).expect("Unable to read input");
    let a:f32 = input8.trim().parse().expect("Unable to input");

    let area_parallelogram:f32 = (b)*(a);

    println!("The area of the parallelogram is {}" ,area_parallelogram);
}

fn cube(){
    println!("Input length of the side:");
    let mut input9 = String::new();
    io::stdin().read_line(&mut input9).expect("Unable to read input");
    let l:f32 = input9.trim().parse().expect("Unable to input");

    let area_cube:f32 = 6.0 * (l * l);

    println!("The area of the cube is {}" ,area_cube);
}

fn volume(){
    let pi:f32 = 3.142;

    println!("Input radius:");
    let mut input10 = String::new();
    io::stdin().read_line(&mut input10).expect("Unable to read input");
    let r:f32 = input10.trim().parse().expect("Unable to input");

    println!("Input height:");
    let mut input11 = String::new();
    io::stdin().read_line(&mut input11).expect("Unable to read input");
    let h_2:f32 = input11.trim().parse().expect("Unable to input");

    let volume_cylinder:f32 = (pi)*(r * r * h_2);

    println!("The volume of cylinder is {}" ,volume_cylinder);
}





