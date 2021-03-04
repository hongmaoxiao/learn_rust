fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x);

    // let spaces = "     ";
    // let spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);

    // let mut spaces = "     ";
    // spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);

    // for number in (1..10) {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
