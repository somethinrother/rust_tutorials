use std::convert::From;
use std::string::ToString;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main() {
    // FROM
    println!("");
    println!("********** FROM **********");
    println!("");

    let num = Number::from(30);
    println!("My number is {:?}", num);

    // INTO
    println!("");
    println!("********** INTO **********");
    println!("");

    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);

    // TOSTRING
    println!("");
    println!("********** TOSTRING **********");
    println!("");

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // PARSING A STRING
    println!("");
    println!("********** PARSING A STRING **********");
    println!("");
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
