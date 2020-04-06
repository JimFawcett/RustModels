struct Helper { s:String, }

impl Helper {
    fn set_string(&mut self, s:&str) {
        self.s = s.to_string();
    }
    fn get_string(&self) -> &String {
        &self.s
    }
}

fn main() {
    print!("\n  Hello, world!");
    let mut h = Helper { s:" ".to_string(), };
    h.set_string("Hi from Helper object");
    print!("\n  {}", h.get_string());
    let s = String::from("Hello from Helper object");
    h.set_string(&s);
    print!("\n  {}", h.get_string());
    println!("\n  That's all folks!\n");
}
