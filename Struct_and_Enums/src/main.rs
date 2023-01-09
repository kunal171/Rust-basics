#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    ColorNone,
}

impl Person {
    pub fn print(self) -> String {
        format!("name = {}, age = {} had {} children",
            self.name, self.age, self.children
        )
    }
}

fn main() {
    let p = Person {
        name: "Kunal".to_string(),
        age: 24,
        children: 0
    };

    let c = Color::Red;
    match c {
        Color::Red => println!("It is red"),
        Color::Green => println!("It is green"),
        Color::Blue => println!("It is blue"),
        _ => println!("It's not a color"),
    }

    println!("Hello, , people, from {:?} ", p);
}
