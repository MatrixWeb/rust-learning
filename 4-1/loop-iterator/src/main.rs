fn main() {
    println!("Hello, world!");
    // learn hash map
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));
    let book = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));
    let obsolete = "Ancient Roman History";
    reviews.remove(obsolete);
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
    // use for while
    let mut counter = 1;
    let stop_loop = loop {
        counter *= 2;
        if counter > 10 {
            break counter;
        }
    };
    println!("Break the loop at counter = {}.", stop_loop);
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
    for number in 0..5 {
        println!("{}", number * 2);
    }
}
