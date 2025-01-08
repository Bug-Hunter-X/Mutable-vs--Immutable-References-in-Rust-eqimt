fn main() {
    let mut x = 5;
    { // Creating a scope for the mutable reference
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modifying x through y is fine
    }
    let z = &x; // z is an immutable reference to x - safe after mutable ref is out of scope
    println!("x = {}", x);
}