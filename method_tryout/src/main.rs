
impl Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep{ name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn say_name(&self) {
        println!("My name is {}", self.name);
    }
    fn set_name(&mut self, new_name: &'static str) {
        self.name = new_name;
    }
    //The above code is a sugar version, which is equivalence to
    //fn set_name(self:mut &Self, new_name: &'static str) {
    //    self.name = new_name;
    //}
}
fn main() {

    let me: Sheep = Sheep::new("pon");
    println!("Sheep name is {}", me.name());
    me.say_name();
}