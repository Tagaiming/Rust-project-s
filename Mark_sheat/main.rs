use std::io;

fn main() {
    println!("Enter your name");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Somthik is wrong");

    let user = input.trim();

    println!("Enter your Number");
    let mut num_input = String::new();
    io::stdin()
        .read_line(&mut num_input)
        .expect("Somthink is wrong");
    let num: i32 = num_input.trim().parse().expect("Somthink is wrong!");
    if num <= 100 && num >= 80 {
        println!("You got A+: {}", user.to_uppercase());
    } else if num <= 79 && num >= 70 {
        println!("You got A: {}", user.to_uppercase());
    } else if num <= 69 && num >= 60 {
        println!("You got A-: {}", user.to_uppercase());
    } else if num <= 59 && num >= 50 {
        println!("You got B: {}", user.to_uppercase());
    } else if num <= 49 && num >= 40 {
        println!("You got C: {}", user.to_uppercase());
    } else if num <= 39 && num >= 33 {
        println!("You got D :{}", user.to_uppercase());
    } else if num <= 32 && num >= 00 {
        println!("You Fail exam: {}", user.to_uppercase());
    } else {
        println!("Somthink is wrong!: {}", user.to_uppercase())
    }
}
