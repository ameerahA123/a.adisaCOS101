struct Laptop{
    hp: u32,
    ibm:u32,
    toshiba:u32,
    dell:u32
}

impl Laptop{
    fn price(&self)->u32{
        3 * self.hp+
        3 * self.ibm+
        3 * self.toshiba+
        3 * self.dell

    }
}


fn main() {
    let laptop1 = Laptop {
        hp:650000,
        ibm:755000,
        toshiba:550000,
        dell:850000,

    };


    println!("The total cost price is {}" ,laptop1.price());
}
