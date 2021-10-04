mod auth;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    let mut user = auth::User::new("tom", "123456");
    println!("the user name is {}", user.get_username());
    user.set_password("admin12345");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
