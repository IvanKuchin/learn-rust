
#[derive(Clone)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u8 {
        self.age
    }

    fn make_clone(&self) -> Self {
        let mut bro = self.clone();
        bro.name = "Bro".to_string();
        bro
    }
}

// impl Clone for Person {
//     fn clone(&self) -> Self {
//         Self {
//             name: self.name.clone(),
//             age: self.age,
//         }
//     }
// }

fn main() {
    let p1 = Person::new("John".to_string(), 20);
    println!("name: {}, age: {}", p1.name(), p1.age());
    let bro = p1.make_clone();
    println!("name: {}, age: {}", bro.name(), bro.age());
}
