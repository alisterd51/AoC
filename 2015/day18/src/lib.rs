#[must_use]
pub fn parse_game(input: &str) -> Vec<Vec<bool>> {
    let mut game = vec![];
    for line in input.lines() {
        let mut game_line = vec![];
        for c in line.chars() {
            match c {
                '#' => game_line.push(true),
                '.' => game_line.push(false),
                _ => {}
            }
        }

        game.push(game_line);
    }

    game
}

fn count_neighbors(game: &[Vec<bool>], y: usize, x: usize, height: usize, width: usize) -> u8 {
    let mut count = 0;

    if y > 0 {
        if x > 0 && game[y - 1][x - 1] {
            count += 1;
        }
        if game[y - 1][x] {
            count += 1;
        }
        if x + 1 < width && game[y - 1][x + 1] {
            count += 1;
        }
    }
    if x > 0 && game[y][x - 1] {
        count += 1;
    }
    if x + 1 < width && game[y][x + 1] {
        count += 1;
    }
    if y + 1 < height {
        if x > 0 && game[y + 1][x - 1] {
            count += 1;
        }
        if game[y + 1][x] {
            count += 1;
        }
        if x + 1 < width && game[y + 1][x + 1] {
            count += 1;
        }
    }

    count
}

#[must_use]
pub fn solve_part_1(game: &[Vec<bool>], steps: usize) -> usize {
    let height = game.len();
    let width = game[0].len();
    let mut game = game.to_vec();
    for _ in 0..steps {
        let mut new_game = vec![vec![false; width]; height];
        for y in 0..height {
            for x in 0..width {
                let neighbors = count_neighbors(&game, y, x, height, width);
                new_game[y][x] = neighbors == 3 || (neighbors == 2 && game[y][x]);
            }
        }

        game = new_game;
    }
    game.iter().flatten().filter(|&&b| b).count()
}

#[must_use]
pub fn solve_part_2(game: &[Vec<bool>], steps: usize) -> usize {
    let height = game.len();
    let width = game[0].len();
    let mut game = game.to_vec();
    game[0][0] = true;
    game[0][width - 1] = true;
    game[height - 1][0] = true;
    game[height - 1][width - 1] = true;
    for _ in 0..steps {
        let mut new_game = vec![vec![false; width]; height];
        for y in 0..height {
            for x in 0..width {
                let neighbors = count_neighbors(&game, y, x, height, width);
                new_game[y][x] = neighbors == 3 || (neighbors == 2 && game[y][x]);
            }
        }

        game = new_game;
        game[0][0] = true;
        game[0][width - 1] = true;
        game[height - 1][0] = true;
        game[height - 1][width - 1] = true;
    }
    game.iter().flatten().filter(|&&b| b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from(
            ".#.#.#
...##.
#....#
..#...
#.#..#
####..",
        );
        let input = parse_game(&input);
        let result = solve_part_1(&input, 4);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_solve_part_2() {
        let input = String::from(
            "##.#.#
...##.
#....#
..#...
#.#..#
####.#",
        );
        let input = parse_game(&input);
        let result = solve_part_2(&input, 5);
        assert_eq!(result, 17);
    }
}
