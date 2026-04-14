fn is_valid_password(password: &str) -> bool {
    password
        .bytes()
        .zip(password.bytes().skip(1))
        .zip(password.bytes().skip(2))
        .any(|((a, b), c)| a + 1 == b && b + 1 == c)
        && !password.chars().any(|c| matches!(c, 'i' | 'o' | 'l'))
        && password
            .as_bytes()
            .chunk_by(|a, b| a == b)
            .map(|groupe| groupe.len() / 2)
            .sum::<usize>()
            >= 2
}

#[must_use]
pub fn solve_part_1(password: &str) -> Option<String> {
    let mut bytes = password.to_string().into_bytes();

    loop {
        if bytes.iter().all(|&b| b == b'z') {
            break;
        }
        for c in bytes.iter_mut().rev() {
            if *c == b'z' {
                *c = b'a';
            } else {
                *c += 1;
                break;
            }
        }
        if let Ok(new_password) = String::from_utf8(bytes.clone())
            && is_valid_password(&new_password)
        {
            return Some(new_password);
        }
    }
    None
}

#[must_use]
pub fn solve_part_2(password: &str) -> Option<String> {
    if let Some(password) = solve_part_1(password)
        && let Some(password) = solve_part_1(&password)
    {
        return Some(password);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from("hijklmmn");
        assert!(!is_valid_password(&input));
        let input = String::from("abbceffg");
        assert!(!is_valid_password(&input));
        let input = String::from("abbcegjk");
        assert!(!is_valid_password(&input));
        let input = String::from("abcdffaa");
        assert!(is_valid_password(&input));
        let input = String::from("ghjaabcc");
        assert!(is_valid_password(&input));

        let input = String::from("abcdefgh");
        let output = String::from("abcdffaa");
        assert_eq!(solve_part_1(&input), Some(output));

        let input = String::from("ghijklmn");
        let output = String::from("ghjaabcc");
        assert_eq!(solve_part_1(&input), Some(output));
    }

    #[test]
    fn example_solve_part_2() {}
}
