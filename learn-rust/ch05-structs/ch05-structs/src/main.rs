fn main() {
    let c = Commodity {
        id: 0,
        name: String::from("商品A"),
        cname: String::from("commodity A"),
        price: 0.1,
        desc: String::from("commodity A"),
    };

    print!("{}", c.price());
}

struct Commodity {
    id: u32,
    name: String,
    cname: String,
    price: f64,
    desc: String,
}

impl Commodity {
    fn new(id: u32, name: String, cname: String, price: f64, desc: String) -> Commodity {
        Commodity {
            id,
            name,
            cname,
            price,
            desc,
        }
    }
    fn price(&self) -> f64 {
        self.price * 9.0
    }
}
