// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    number = "3"; // don't rename this variable
    let number: i32 = number.parse().expect("Not a valid number");
    println!("Number plus two is : {}", number + 2);
}
