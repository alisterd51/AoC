enum Switch {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Coord {
    x: usize,
    y: usize,
}

pub struct Instruction {
    switch: Switch,
    a: Coord,
    b: Coord,
}

#[must_use]
pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in input.lines() {
        let (switch, instruction) = if let Some(instruction) = line.strip_prefix("turn on ") {
            (Switch::TurnOn, instruction)
        } else if let Some(instruction) = line.strip_prefix("turn off ") {
            (Switch::TurnOff, instruction)
        } else if let Some(instruction) = line.strip_prefix("toggle ") {
            (Switch::Toggle, instruction)
        } else {
            continue;
        };
        if let Some((a, b)) = instruction.split_once(" through ")
            && let Some((a_x, a_y)) = a.split_once(',')
            && let Ok(a_x) = a_x.parse::<usize>()
            && let Ok(a_y) = a_y.parse::<usize>()
            && let Some((b_x, b_y)) = b.split_once(',')
            && let Ok(b_x) = b_x.parse::<usize>()
            && let Ok(b_y) = b_y.parse::<usize>()
        {
            instructions.push(Instruction {
                switch,
                a: Coord { x: a_x, y: a_y },
                b: Coord { x: b_x, y: b_y },
            });
        }
    }

    instructions
}

#[must_use]
pub fn solve_part_1(instructions: &[Instruction]) -> usize {
    let mut map = vec![[false; 1000]; 1000].into_boxed_slice();

    for instruction in instructions {
        for y in instruction.a.y..=instruction.b.y {
            for x in instruction.a.x..=instruction.b.x {
                match instruction.switch {
                    Switch::TurnOn => map[y][x] = true,
                    Switch::TurnOff => map[y][x] = false,
                    Switch::Toggle => map[y][x] = !map[y][x],
                }
            }
        }
    }

    map.iter().flatten().filter(|&&b| b).count()
}

#[must_use]
pub fn solve_part_2(instructions: &[Instruction]) -> usize {
    let mut map = vec![[0usize; 1000]; 1000].into_boxed_slice();

    for instruction in instructions {
        for y in instruction.a.y..=instruction.b.y {
            for x in instruction.a.x..=instruction.b.x {
                match instruction.switch {
                    Switch::TurnOn => map[y][x] += 1,
                    Switch::TurnOff => map[y][x] = map[y][x].saturating_sub(1),
                    Switch::Toggle => map[y][x] += 2,
                }
            }
        }
    }

    map.iter().flatten().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from("turn on 0,0 through 999,999");
        let input = parse_instructions(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 1000000);
        let input = String::from("toggle 0,0 through 999,0");
        let input = parse_instructions(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 1000);
        let input = String::from("turn off 499,499 through 500,500");
        let input = parse_instructions(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_solve_part_2() {
        let input = String::from("turn on 0,0 through 0,0");
        let input = parse_instructions(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, 1);
        let input = String::from("toggle 0,0 through 999,999");
        let input = parse_instructions(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, 2000000);
    }
}
