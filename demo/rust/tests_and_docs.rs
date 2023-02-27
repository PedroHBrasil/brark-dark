// This is a simple Rust program that demonstrates comments, docstrings, and unit tests.

/// This function returns the square of a number.
///
/// # Examples
///
/// ```
/// let result = square(3);
/// assert_eq!(result, 9);
/// ```
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    // This is a comment. It's ignored by the compiler.
    // Here we're creating some variables and calling the square function.
    let x = 5;
    let result = square(x);

    // This is also a comment. It's ignored by the compiler.
    // Here we're printing the result of the square function.
    println!("The square of {} is {}.", x, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(2), 4);
        assert_eq!(square(3), 9);
        assert_eq!(square(4), 16);
        assert_eq!(square(5), 25);
    }
}
