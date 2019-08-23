struct Pon {
    name: String
}

impl Pon {
    fn new(name: &str) -> Pon {
        Pon { name: name.to_string() }
    }

    fn say_hi(&self) {
        println!("Hi, my name is {}", self.name);
    }

    fn prefix_name(&mut self, prefix: &str) {
        let mut prefix_string = String::from(prefix);
        prefix_string.push_str(&self.name);
        self.name = prefix_string;
    }
}

fn main() {
    let mut pon = Pon::new("pon");
    pon.say_hi();
    pon.prefix_name("Mr.");
    pon.say_hi();
}
