#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn main() {
    println!("Hello, world!");
    // ownership
    let s = String::from("Hello, world!");
    process(s.clone());
    process(s);
    let mut greeting = String::from("hello");
    change(&mut greeting);
    println!("greeting: {}", greeting);
    // lifetime
    let magic2 = String::from("shen");
    {
        let magic1 = String::from("yuan");
        let result;
        {
            result = longest_word(&magic2, &magic1);
        }
        println!("the longest word: {}", result);
    }
    println!("magic2 {}", magic2);
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);
}

fn process(s: String) { println!("use {}", s);}

fn change(s: &mut String) {s.push_str(", world");}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
