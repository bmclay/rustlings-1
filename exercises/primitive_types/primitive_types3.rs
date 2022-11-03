// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.



fn main() {
    let mut a = Vec::new();

    let i:i32 = 100;
    let mut j:i32 = 0;

    while j<i{
        a.push(j);
        j += 1;
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
