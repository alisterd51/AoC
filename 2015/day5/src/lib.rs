#[must_use]
pub fn parse_string(input: &str) -> Vec<String> {
    let mut strings = vec![];
    for line in input.lines() {
        if !line.is_empty() {
            strings.push(String::from(line));
        }
    }

    strings
}

fn three_vowels(string: &str) -> bool {
    let count = string
        .chars()
        .filter(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count();
    count >= 3
}

fn double_letter(string: &str) -> bool {
    string
        .chars()
        .zip(string.chars().skip(1))
        .any(|(a, b)| a == b)
}

fn disallowed_substrings(string: &str) -> bool {
    string.contains("ab") || string.contains("cd") || string.contains("pq") || string.contains("xy")
}

#[must_use]
pub fn solve_part_1(strings: &[String]) -> u64 {
    let mut nice_string = 0;

    for string in strings {
        if three_vowels(string) && double_letter(string) && !disallowed_substrings(string) {
            nice_string += 1;
        }
    }

    nice_string
}

fn pair_twice(string: &str) -> bool {
    for (index, ab) in string.chars().zip(string.chars().skip(1)).enumerate() {
        for cd in string
            .chars()
            .skip(index + 2)
            .zip(string.chars().skip(index + 3))
        {
            if ab.0 == cd.0 && ab.1 == cd.1 {
                return true;
            }
        }
    }
    false
}

fn gap_repeat(string: &str) -> bool {
    string
        .chars()
        .zip(string.chars().skip(2))
        .any(|(a, b)| a == b)
}

#[must_use]
pub fn solve_part_2(strings: &[String]) -> u64 {
    let mut nice_string = 0;

    for string in strings {
        if pair_twice(string) && gap_repeat(string) {
            nice_string += 1;
        }
    }

    nice_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from("ugknbfddgicrmopn");
        let input = parse_string(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 1);
        let input = String::from("aaa");
        let input = parse_string(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 1);
        let input = String::from("jchzalrnumimnmhp");
        let input = parse_string(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
        let input = String::from("haegwjzuvuyypxyu");
        let input = parse_string(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
        let input = String::from("dvszwmarrgswjxmb");
        let input = parse_string(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
        let input = String::from(
            "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb",
        );
        let input = parse_string(&input);
        println!("{input:?}");
        let result = solve_part_1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn other_solve_part_1() {
        let input = String::from("aaab");
        let input = parse_string(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_solve_part_2() {
        // let input = String::from("qjhvhtzxzqqjkmpb");
        // let input = parse_string(&input);
        // let result = solve_part_2(&input);
        // assert_eq!(result, 1);
        // let input = String::from("xxyxx");
        // let input = parse_string(&input);
        // let result = solve_part_2(&input);
        // assert_eq!(result, 1);
        // let input = String::from("uurcxstgmygtbstg");
        // let input = parse_string(&input);
        // let result = solve_part_2(&input);
        // assert_eq!(result, 0);
        let input = String::from("ieodomkazucvgmuy");
        let input = parse_string(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, 0);
    }
}

// part1 164 trop bas
