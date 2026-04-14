#[must_use]
pub fn parse_sequences(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

#[must_use]
pub fn solve(sequences: &[Vec<u32>], deep: usize) -> usize {
    let mut result = 0;
    for sequence in sequences {
        let mut sequence = sequence.clone();
        for _ in 0..deep {
            let mut new_sequence = vec![];
            let mut current = None;
            for digit in sequence {
                current = match current {
                    None => Some((digit, 1)),
                    Some((prev_digit, len)) => {
                        if prev_digit == digit {
                            Some((digit, len + 1))
                        } else {
                            new_sequence.push(len);
                            new_sequence.push(prev_digit);
                            Some((digit, 1))
                        }
                    }
                }
            }
            if let Some((digit, len)) = current {
                new_sequence.push(len);
                new_sequence.push(digit);
            }

            sequence = new_sequence;
        }

        result += sequence.len();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve() {
        let input = String::from("1");
        let input = parse_sequences(&input);
        let result = solve(&input, 1);
        assert_eq!(result, 2);
        let result = solve(&input, 2);
        assert_eq!(result, 2);
        let result = solve(&input, 3);
        assert_eq!(result, 4);
        let result = solve(&input, 4);
        assert_eq!(result, 6);
        let result = solve(&input, 5);
        assert_eq!(result, 6);
    }
}
