// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// I AM NOT DONE
//     let a = [0..100];
//     println!("{}", a.len());
// Would print 1 here

fn main() {
    let a = 0..100;
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
