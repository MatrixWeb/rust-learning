#[derive(Debug)]
struct DivisionByZeroError;

fn main() {
    println!("Hello, world!");
    //panic!("Farewell!!");
    // learn option
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
    // if-let
    let a_number : Option<u8> = Some(7);
    if let Some(7) = a_number{
        println!("That's my lucky number!");
    }
    // unwrap expect
    let gift = Some("Candy");
    assert_eq!(gift.unwrap(), "Candy");
    assert_eq!(None.unwrap_or("Cat"), "Cat");
    //let b: Option<&str> = None;
    //b.expect("fruits are healthy");
    // learn Result
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}