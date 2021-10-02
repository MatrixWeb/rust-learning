
struct Student { name: String, level: u8, remote: bool}
struct Grades(char, char, char, char, f32);

#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64
}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress)
}

fn divide_by_5(num: i32) -> i32 {
    num / 5
}

fn goodbye(message: &str) -> bool {
    println!("\n{}", message);
    return true
}

fn main() {
    println!("Hello, world!");
    println!("the first letter is {}, end is {}", 'A', 'Z');
    // use varibale
    let a_number = 10;
    let a_word = "Ten";
    println!("nerber {}, word: {}", a_number, a_word);
    // use mut make varibale mutable
    let mut a_mut_num = 19;
    println!("a mut num before change: {}", a_mut_num);
    a_mut_num = 15;
    println!("a mut num: {}", a_mut_num);
    // basic types
    let b_number: u32 = 14;
    println!("the b u32 number: {}", b_number);
    let number_64 = 4.0;
    let number_32: f32 = 5.0;
    println!("decimal  number f64: {}, f32 : {}", number_64, number_32);
    let is_bigger = 1 > 4;
    println!("is 1 > 4 : {}", is_bigger);
    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃';
    println!("upper case s: {}, lower case f: {}, smiley face: {}", uppercase_s, lowercase_f, smiley_face);
    // text type
    let character_1: char = 'S';
    let character_2: char = 'f';
    let string_1 = "miley ";
    let string_2 = "ace";
    println!("{} is a {}{}{}{}", smiley_face, character_1, string_1, character_2, string_2);
    // tuple
    let tuple_e = ('E', 5i32, true);
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2); 
    // sturct
    let user_1 = Student {name: String::from("Constance Sharm"), remote: true, level: 2};
    let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
         user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
         user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
    // enum
    let click = MouseClick {
        x: 100,
        y: 250
    };
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    let we_load = WebEvent::WELoad(true);
    let we_click= WebEvent::WEClick(click);
    let we_key = WebEvent::WEKeys(keys);
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
    // function
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
    
    let formal = "Formal: Good bye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);
}
