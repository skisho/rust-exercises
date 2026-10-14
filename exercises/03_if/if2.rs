// if2.rs
//
// Classic FizzBuzz. Make the tests pass:
//   - multiples of 3 give "Fizz"
//   - multiples of 5 give "Buzz"
//   - multiples of both give "FizzBuzz"
//   - anything else gives the number itself, as a String

// I AM NOT DONE

fn fizzbuzz(n: u32) -> String {
    if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        "?".to_string()
    }
}

fn main() {
    for n in 1..=15 {
        println!("{}", fizzbuzz(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_numbers() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(7), "7");
    }

    #[test]
    fn fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn buzz() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(20), "Buzz");
    }

    #[test]
    fn fizzbuzz_both() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(45), "FizzBuzz");
    }
}
