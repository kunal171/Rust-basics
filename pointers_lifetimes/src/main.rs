#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person {name, age}
    }

    pub fn greet(&self)-> String {
        format!("Hi my name is {}", self.name)
    }

    pub fn age_up(&mut self, n: i32) {
        self.age += n
    }

    pub fn dropme(self) {}
    
}

fn main() {

    let mut p = Person::new("John".to_string(), 35);
    p.age_up(3);
    let s = p.greet();
    println!("{}", s);

    let a = get_age(&p);
    println!("person age is {}", a);

    let s2 = p.greet();
    println!("really: {}", s2);
}

pub fn get_age(s: &Person) -> &i32 {
    &s.age
}