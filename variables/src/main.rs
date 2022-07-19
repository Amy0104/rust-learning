/**
 * The difference between const and let
 * 1. let is allowed to be used with mut, but const is not allowed
 * 2. const can be declared in any scope
 * 3. const set only to a constant expression, not the result of a value
 */
/*
 * The difference between shadowing and mut
*/
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
