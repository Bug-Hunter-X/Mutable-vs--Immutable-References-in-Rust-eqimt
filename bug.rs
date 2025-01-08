fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modifying x through y is fine
    // *z += 1; // This will result in a compile-time error
    println!("x = {}", x);
}