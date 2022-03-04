// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143?

// Solution of third Euler problem.
pub fn euler_3(mut input: i64) -> i64 {
    let mut pm = 2;

    while input != 1 {
        if input % pm == 0 {
            input = input / pm;
        } else {
            pm += 1;
        }
    }

    pm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_3() {
        let parameters = &[
            (2, 2),
            (3, 3),
            (5, 5),
            (7, 7),
            (8, 2),
            (13195, 29),
            (600851475143, 6857),
        ];

        for (input, output) in parameters {
            assert_eq!(euler_3(*input), *output);
        }
    }
}
