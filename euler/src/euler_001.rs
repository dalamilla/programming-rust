// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

// Solution of first Euler problem.
pub fn euler_1(input: i32) -> i32 {
    let mut sum = 0;
    let mut i = 0;

    while i < input {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
        i += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_1() {
        let parameters = &[
            (1000, 233168),
            (49, 543),
            (8456, 16687353),
            (19564, 89301183),
        ];

        for (input, output) in parameters {
            assert_eq!(euler_1(*input), *output);
        }
    }
}
