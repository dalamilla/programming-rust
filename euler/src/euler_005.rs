// 2520 is the smallest number that can be divided by each of
// the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible
// by all of the numbers from 1 to 20?

// Solution of fifth Euler problem.
pub fn euler_5(input: i32) -> i32 {
    let mut found = true;
    let mut number = 0;

    while found {
        let mut i = 1;
        number += input;

        while number % i == 0 && i <= input {
            if i == input {
                found = false;
            }
            i += 1;
        }
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_5() {
        let parameters = &[(5, 60), (7, 420), (10, 2520), (13, 360360), (20, 232792560)];

        for (input, output) in parameters {
            assert_eq!(euler_5(*input), *output);
        }
    }
}
