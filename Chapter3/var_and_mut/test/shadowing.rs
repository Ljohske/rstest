fn main() {
    let x = 5;

    let x = x + 1;
    let x = x * 2;

    // variable x is shadowed
    println!("The value of x is: {}", x);

    /*
     * Shadowing is different from mutable variables because:
     *
     * 1. when shadowed, variables become immutable after re-declaration
     * 2. shadowing is basically creating a different variable with the same
     * name, so you can assign values of different types to that variable.
     */

    // compile error
    /* let mut spaces = "  ";
     * spaces = spaces.len();
     */

    let spaces = "  ";
    let spaces = spaces.len();
    println!("number of spaces is: {}", spaces)
}
