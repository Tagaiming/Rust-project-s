use std::io;

fn main() {
    println!("*** guessing game********");

    loop {
        let num = 20;
        println!("Enter your number which is 1-20");
        let mut user = String::new();

        io::stdin()
            .read_line(&mut user)
            .expect("Error reading user");

        let user_number: i32 = user.trim().parse().expect("Please type a number!");
        if user_number == num {
            println!("You won the game\n");
            break;
        } else {
            println!("Try again\n");
        }
    }
}
