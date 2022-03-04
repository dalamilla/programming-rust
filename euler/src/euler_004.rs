// A palindromic number reads the same both ways. The largest palindrome made from
// the product of two 2-startDigit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-startDigit numbers.

// Solution of fourth Euler problem.
pub fn euler_4(input: u32) -> i32 {
    let start_digit = 10i32.pow(input - 1);
    let end_digit = 10i32.pow(input);

    let mut max = 0;
    let mut i = start_digit;

    while i < end_digit {
        let mut j = start_digit;
        while j < end_digit {
            if is_palindrome(i * j) && i * j > max {
                max = i * j
            }
            j += 1;
        }
        i += 1;
    }

    max
}

// Helper function of fourth Euler problem.
fn is_palindrome(input: i32) -> bool {
    let pali = input.to_string();
    return pali == pali.chars().rev().collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_4() {
        let parameters = &[(2, 9009), (3, 906609)];

        for (input, output) in parameters {
            assert_eq!(euler_4(*input), *output);
        }
    }
}
