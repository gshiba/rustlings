// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.


fn main() {
    let a = vec![3; 100];
    let a = "yyxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    // let a = ["hi"; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!, {}", a.len());
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
