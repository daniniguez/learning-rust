// Variables by default are immutable meaning that once a value is bound
// to a name you cannot change that value.
// The compiler then guarantees that when you state a value won't change
// it really won't.
fn main () {
    // let x = 5; This is a default immutable variable. Error is expected
    // if the program runs with an immutable variable.

    let mut x = 5; // Adding a mutable variable.
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
