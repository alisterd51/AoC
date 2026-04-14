#![allow(clippy::missing_panics_doc, clippy::unwrap_used)]

use regex::Regex;

#[must_use]
pub fn parse_lines(input: &str) -> Vec<String> {
    let mut lines = vec![];

    for line in input.lines() {
        lines.push(line.to_string());
    }

    lines
}

#[must_use]
pub fn solve_part_1(lines: &[String]) -> usize {
    let mut result = 0;
    let re = Regex::new(r#"\\x[0-9a-fA-F]{2}|\\"|\\\\"#).unwrap();

    for line in lines {
        let mut parsed_line = line.as_str();
        if let Some(line) = line.strip_prefix(r#"""#)
            && let Some(line) = line.strip_suffix(r#"""#)
        {
            parsed_line = line;
        }
        let parsed_line = re.replace_all(parsed_line, |caps: &regex::Captures| {
            let correspondance = &caps[0];

            if correspondance.starts_with(r"\x") {
                ".".to_string()
            } else {
                match correspondance {
                    r#"\""# => r#"""#.to_string(),
                    r"\\" => r"\".to_string(),
                    _ => correspondance.to_string(),
                }
            }
        });
        result += line.len() - parsed_line.len();
    }
    result
}

#[must_use]
pub fn solve_part_2(lines: &[String]) -> usize {
    let mut result = 0;

    for line in lines {
        let line_len = line.len();
        let mut new_len = 2;
        for c in line.chars() {
            new_len += if c == '"' || c == '\\' { 2 } else { 1 }
        }

        result += new_len - line_len;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from("\"\"");
        let input = parse_lines(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 2 - 0);
        let input = String::from("\"abc\"");
        let input = parse_lines(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 5 - 3);
        let input = String::from("\"aaa\\\"aaa\"");
        let input = parse_lines(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 10 - 7);
        let input = String::from("\"\\x27\"");
        let input = parse_lines(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 6 - 1);
        let input = String::from(
            "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"",
        );
        let input = parse_lines(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, (2 + 5 + 10 + 6) - (0 + 3 + 7 + 1));
    }

    #[test]
    fn other_solve_part_1() {
        let input = String::from("\"\\\\xmy\"");
        let input = parse_lines(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 7 - 4);
    }

    #[test]
    fn example_solve_part_2() {
        let input = String::from("\"\"");
        let input = parse_lines(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, 6 - 2);
        let input = String::from("\"abc\"");
        let input = parse_lines(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, 9 - 5);
        let input = String::from("\"aaa\\\"aaa\"");
        let input = parse_lines(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, 16 - 10);
        let input = String::from("\"\\x27\"");
        let input = parse_lines(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, 11 - 6);
        let input = String::from(
            "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"",
        );
        let input = parse_lines(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, (6 + 9 + 16 + 11) - (2 + 5 + 10 + 6));
    }
}
