struct Pointe<T, U> {
    x: T,
    y: U,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

trait AsJson {
    fn as_json(&self) -> String;
}

fn send_data_as_json(value: &impl AsJson) {
    println!("Sending JSON data to server...");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}

struct Person {
    name: String,
    age: u8,
    favorite_fruit: String,
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter {
            length: length,
            count: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
            self.name, self.age, self.favorite_fruit
        )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
	    format!(
	        r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
	        self.name, self.color, self.likes_petting
	    )
    }
}

use std::fmt;
impl fmt::Display for Point {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    println!("Hello, world!");
    // generic data type
    let float_and_string = Pointe { x: 6, y: "hey"};
    println!("inner float and string: {}, {}", float_and_string.x, float_and_string.y);
    // traits
    let circle = Circle { radius: 5.0 };
    let retangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", retangle.area());
    // use derive
    let p1 = Point { x: 1, y: 2};
    let p2 = Point { x: 3, y: 4};
    if p1 == p2 {
        println!("point equal!");
    } else {
        println!("point unequal!");
    }
    println!("{}", p1);
    println!("{:?}", p2);

    let laura = Person {
    	name: String::from("Laura"),
	    age: 31,
	    favorite_fruit: String::from("apples"),
    };

    let fido = Dog {
	    name: String::from("Fido"),
	    color: String::from("Black"),
	    likes_petting: true,
    };

    send_data_as_json(&laura);
    send_data_as_json(&fido);
    let counter = Counter::new(6);
    for number in counter {
        println!("counter: {}", number);
    }
}
