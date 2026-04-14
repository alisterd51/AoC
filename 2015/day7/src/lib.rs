use std::collections::HashMap;

enum Wire {
    Int(u16),
    Index(String),
}

enum Input {
    Signal(Wire),
    Not(Wire),
    Or((Wire, Wire)),
    And((Wire, Wire)),
    LShift((Wire, u8)),
    RShift((Wire, u8)),
}

pub struct Operator {
    input: Input,
    output: String,
}

fn parse_wire(wire: &str) -> Wire {
    if let Ok(value) = wire.parse::<u16>() {
        Wire::Int(value)
    } else {
        Wire::Index(wire.to_string())
    }
}

#[must_use]
pub fn parse_operators(input: &str) -> Vec<Operator> {
    let mut operators = vec![];

    for line in input.lines() {
        let Some((input, output)) = line.split_once(" -> ") else {
            continue;
        };
        let parts: Vec<&str> = input.split_whitespace().collect();

        let input = match parts.as_slice() {
            ["NOT", wire] => Input::Not(parse_wire(wire)),
            [left, "OR", right] => Input::Or((parse_wire(left), parse_wire(right))),
            [left, "AND", right] => Input::And((parse_wire(left), parse_wire(right))),
            [wire, "LSHIFT", shift] => {
                let Ok(shift) = shift.parse::<u8>() else {
                    continue;
                };
                Input::LShift((parse_wire(wire), shift))
            }
            [wire, "RSHIFT", shift] => {
                let Ok(shift) = shift.parse::<u8>() else {
                    continue;
                };
                Input::RShift((parse_wire(wire), shift))
            }
            [wire] => Input::Signal(parse_wire(wire)),
            _ => continue,
        };
        let operator = Operator {
            input,
            output: output.to_string(),
        };

        operators.push(operator);
    }

    operators
}

fn eval_wire(
    operators: &HashMap<&str, &Input>,
    cache: &mut HashMap<String, u16>,
    wire: &Wire,
) -> Option<u16> {
    match wire {
        Wire::Int(value) => Some(*value),
        Wire::Index(index) => recursive_solve(operators, cache, index),
    }
}

fn recursive_solve(
    operators: &HashMap<&str, &Input>,
    cache: &mut HashMap<String, u16>,
    index: &str,
) -> Option<u16> {
    if let Some(&value) = cache.get(index) {
        return Some(value);
    }
    let result = if let Some(operator) = operators.get(index) {
        match operator {
            Input::Signal(wire) => eval_wire(operators, cache, wire),
            Input::Not(wire) => eval_wire(operators, cache, wire).map(|value| !value),
            Input::Or((wire_0, wire_1)) => {
                let wire_0 = eval_wire(operators, cache, wire_0)?;
                let wire_1 = eval_wire(operators, cache, wire_1)?;

                Some(wire_0 | wire_1)
            }
            Input::And((wire_0, wire_1)) => {
                let wire_0 = eval_wire(operators, cache, wire_0)?;
                let wire_1 = eval_wire(operators, cache, wire_1)?;

                Some(wire_0 & wire_1)
            }
            Input::LShift((wire, shift)) => {
                let wire = eval_wire(operators, cache, wire);

                wire.map(|wire| wire << shift)
            }
            Input::RShift((wire, shift)) => {
                let wire = eval_wire(operators, cache, wire);

                wire.map(|wire| wire >> shift)
            }
        }
    } else {
        None
    };
    if let Some(result) = result {
        cache.insert(index.to_string(), result);
    }

    result
}

#[must_use]
pub fn solve_part_1(operators: &[Operator], index: &str) -> u16 {
    let operators: HashMap<&str, &Input> = operators
        .iter()
        .map(|operator| (operator.output.as_str(), &operator.input))
        .collect();
    let mut cache = HashMap::new();
    recursive_solve(&operators, &mut cache, index).unwrap_or_default()
}

#[must_use]
pub fn solve_part_2(operators: &[Operator], index: &str) -> u16 {
    let operators: HashMap<&str, &Input> = operators
        .iter()
        .map(|operator| (operator.output.as_str(), &operator.input))
        .collect();
    let mut cache = HashMap::new();
    let first_result = recursive_solve(&operators, &mut cache, index).unwrap_or_default();
    let mut cache = HashMap::new();
    cache.insert("b".to_string(), first_result);
    recursive_solve(&operators, &mut cache, index).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from(
            "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        );
        let input = parse_operators(&input);
        let result = solve_part_1(&input, "d");
        assert_eq!(result, 72);
        let result = solve_part_1(&input, "e");
        assert_eq!(result, 507);
        let result = solve_part_1(&input, "f");
        assert_eq!(result, 492);
        let result = solve_part_1(&input, "g");
        assert_eq!(result, 114);
        let result = solve_part_1(&input, "h");
        assert_eq!(result, 65412);
        let result = solve_part_1(&input, "i");
        assert_eq!(result, 65079);
        let result = solve_part_1(&input, "x");
        assert_eq!(result, 123);
        let result = solve_part_1(&input, "y");
        assert_eq!(result, 456);
    }

    #[test]
    fn example_solve_part_2() {}
}
