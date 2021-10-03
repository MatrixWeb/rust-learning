fn main() {
    println!("Hello, world!");
    // use array (only values can change)
    let mut days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let bytes = [0;5];
    days[1] = "Mondi";
    println!("day of second: {}, bytes first: {}", days[1], bytes[0]);
    // use vector
    let mut three_nums = vec![15, 3, 46];
    println!("init vec:{:?}", three_nums);
    let zeroes = vec![0;5];
    println!("zeros size: {}", zeroes.len());
    let mut fruits = Vec::new();
    fruits.push("Apple");
    fruits.push("Banana");
    println!("fruits: {:?}", fruits);
    println!("Pop off: {:?}", fruits.pop());
    println!("after pop, fruits: {:?}", fruits);
    three_nums[1] = three_nums[0] + three_nums[2];
    println!("middle three num: {}", three_nums[1]);
    // learn if-else
    let formal = true;
    let greeting = if formal {
        "Good day to you"
    } else {
        "Hey"
    };
    println!("{}", greeting);
    let num = 500; // num variable can be set at some point in the program
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    print!("is out if range: {}", out_of_range);
}
