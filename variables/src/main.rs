// Variables by default are immutable meaning that once a vallue is bound
// to a name you cannot change that value.
// The compiler then guarantees that when you state a value won't change
// it really won't.
fn main () {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
