use std::cmp::{max, min};

pub struct Dimension {
    length: u64,
    width: u64,
    height: u64,
}

#[must_use]
pub fn parse_dimensions(input: &str) -> Vec<Dimension> {
    let mut dimensions = vec![];
    for line in input.lines() {
        if let Some((length, wh)) = line.split_once('x')
            && let Ok(length) = length.parse::<u64>()
            && let Some((width, height)) = wh.split_once('x')
            && let Ok(width) = width.parse::<u64>()
            && let Ok(height) = height.parse::<u64>()
        {
            dimensions.push(Dimension {
                length,
                width,
                height,
            });
        }
    }

    dimensions
}

#[must_use]
pub fn solve_part_1(dimensions: &[Dimension]) -> u64 {
    let mut wrapping_paper = 0;
    for dimension in dimensions {
        let a = dimension.length * dimension.width;
        let b = dimension.width * dimension.height;
        let c = dimension.height * dimension.length;
        let min = min(min(a, b), c);
        wrapping_paper += 2 * a + 2 * b + 2 * c + min;
    }

    wrapping_paper
}

#[must_use]
pub fn solve_part_2(dimensions: &[Dimension]) -> u64 {
    let mut wrapping_paper = 0;
    for dimension in dimensions {
        let max = max(max(dimension.length, dimension.width), dimension.height);
        let ribbon = (dimension.length + dimension.width + dimension.height - max) * 2;
        let bow = dimension.length * dimension.width * dimension.height;

        wrapping_paper += ribbon + bow;
    }

    wrapping_paper
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = "2x3x4";
        let input = parse_dimensions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 58);
        let input = "1x1x10";
        let input = parse_dimensions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 43);
    }

    #[test]
    fn example_solve_part_2() {
        let input = "2x3x4";
        let input = parse_dimensions(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 34);
        let input = "1x1x10";
        let input = parse_dimensions(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 14);
    }
}
