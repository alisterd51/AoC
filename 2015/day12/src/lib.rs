#![allow(clippy::missing_errors_doc)]

pub fn parse_json(input: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(input)
}

fn sum_number(json: &serde_json::Value) -> i64 {
    match json {
        serde_json::Value::Number(number) => number.as_i64().unwrap_or_default(),
        serde_json::Value::Array(values) => values.iter().map(sum_number).sum(),
        serde_json::Value::Object(map) => map.values().map(sum_number).sum(),
        _ => 0,
    }
}

#[must_use]
pub fn solve_part_1(json: &serde_json::Value) -> i64 {
    sum_number(json)
}

fn is_red(json: &serde_json::Value) -> bool {
    matches!(json, serde_json::Value::String(s) if s == "red")
}

fn sum_number_without_red(json: &serde_json::Value) -> i64 {
    match json {
        serde_json::Value::Number(number) => number.as_i64().unwrap_or_default(),
        serde_json::Value::Array(values) => values.iter().map(sum_number_without_red).sum(),
        serde_json::Value::Object(map) if !map.values().any(is_red) => {
            map.values().map(sum_number_without_red).sum()
        }
        _ => 0,
    }
}

#[must_use]
pub fn solve_part_2(json: &serde_json::Value) -> i64 {
    sum_number_without_red(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from("[1,2,3]");
        let input = parse_json(&input).unwrap();
        let result = solve_part_1(&input);
        assert_eq!(result, 6);
        let input = String::from(r#"{"a":2,"b":4}"#);
        let input = parse_json(&input).unwrap();
        let result = solve_part_1(&input);
        assert_eq!(result, 6);
        let input = String::from("[[[3]]]");
        let input = parse_json(&input).unwrap();
        let result = solve_part_1(&input);
        assert_eq!(result, 3);
        let input = String::from(r#"{"a":{"b":4},"c":-1}"#);
        let input = parse_json(&input).unwrap();
        let result = solve_part_1(&input);
        assert_eq!(result, 3);
        let input = String::from(r#"{"a":[-1,1]}"#);
        let input = parse_json(&input).unwrap();
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
        let input = String::from(r#"[-1,{"a":1}]"#);
        let input = parse_json(&input).unwrap();
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
        let input = String::from(r#"[]"#);
        let input = parse_json(&input).unwrap();
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
        let input = String::from(r#"{}"#);
        let input = parse_json(&input).unwrap();
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_solve_part_2() {
        let input = String::from("[1,2,3]");
        let input = parse_json(&input).unwrap();
        let result = solve_part_2(&input);
        assert_eq!(result, 6);
        let input = String::from(r#"[1,{"c":"red","b":2},3]"#);
        let input = parse_json(&input).unwrap();
        let result = solve_part_2(&input);
        assert_eq!(result, 4);
        let input = String::from(r#"{"d":"red","e":[1,2,3,4],"f":5}"#);
        let input = parse_json(&input).unwrap();
        let result = solve_part_2(&input);
        assert_eq!(result, 0);
        let input = String::from(r#"[1,"red",5]"#);
        let input = parse_json(&input).unwrap();
        let result = solve_part_2(&input);
        assert_eq!(result, 6);
    }
}
