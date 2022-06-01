fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x is shadowed by x + 6
    let x = x + 6;
    println!("The value of x is: {}", x);
}
