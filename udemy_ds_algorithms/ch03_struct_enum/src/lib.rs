#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    children: i32,
    favourite_color: Color,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name = {}, age = {} has {} children whose favourite color is {:#?}",
            self.name, self.age, self.children, self.favourite_color
        )
    }
}

#[derive(Debug)]
enum Color {
    Blue(String),
    Green,
    Red,
}

pub fn test_person_and_color() {
    let p = Person {
        name: "Thor".to_string(),
        age: 1500,
        children: 6,
        favourite_color: Color::Blue("marlin".to_string()),
    };

    let color = Color::Blue("whale".to_string());
    match  color {
        Color::Red => println!("It's red"),
        Color::Green => println!("It's green"),
        Color::Blue(s) => println!("It's a blue {}.", s),
    }

    println!("Hello, world, from {},", p.print());

    assert_eq!(1, 1);
}