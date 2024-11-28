// TODO: Fix the compiler error on this function.
fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    }  else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}

fn main() {
    // You can optionally experiment here.
    let result = foo_if_fizz("fizz");
    println!("The result for 'fizz' is: {}", result);

    let result2 = foo_if_fizz("fuzz");
    println!("The result for 'fuzz' is: {}", result2);
    
    let result3 = foo_if_fizz("test");
    println!("The result for 'test' is: {}", result3);
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        // This means that calling `foo_if_fizz` with the argument "fizz" should return "foo".
        assert_eq!(foo_if_fizz("fizz"), "foo");
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar");
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz");
    }
}
