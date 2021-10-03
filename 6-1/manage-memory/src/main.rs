fn main() {
    println!("Hello, world!");
    // ownership
    let s = String::from("Hello, world!");
    process(s.clone());
    process(s);
    let mut greeting = String::from("hello");
    change(&mut greeting);
    println!("greeting: {}", greeting);
}

fn process(s: String) { println!("use {}", s);}

fn change(s: &mut String) {s.push_str(", world");}
