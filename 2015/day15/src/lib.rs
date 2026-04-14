use itertools::Itertools;

pub struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

#[must_use]
pub fn parse_ingredients(input: &str) -> Vec<Ingredient> {
    let mut ingredients = vec![];

    for line in input.lines() {
        if let words = line.split_whitespace().collect::<Vec<&str>>()
            && words.len() == 11
            && let Some(capacity) = words[2].strip_suffix(",")
            && let Ok(capacity) = capacity.parse::<i64>()
            && let Some(durability) = words[4].strip_suffix(",")
            && let Ok(durability) = durability.parse::<i64>()
            && let Some(flavor) = words[6].strip_suffix(",")
            && let Ok(flavor) = flavor.parse::<i64>()
            && let Some(texture) = words[8].strip_suffix(",")
            && let Ok(texture) = texture.parse::<i64>()
            && let Ok(calories) = words[10].parse::<i64>()
        {
            ingredients.push(Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            });
        }
    }

    ingredients
}

#[must_use]
pub fn solve_part_1(ingredients: &[Ingredient]) -> i64 {
    let mut max_score = 0;

    for combination in (0..ingredients.len())
        .map(|_| 0..=100)
        .multi_cartesian_product()
    {
        if combination.iter().sum::<i64>() == 100 {
            let mut sum = Ingredient {
                capacity: 0,
                durability: 0,
                flavor: 0,
                texture: 0,
                calories: 0,
            };

            for (id, ingredient) in ingredients.iter().enumerate() {
                sum.capacity += ingredient.capacity * combination[id];
                sum.durability += ingredient.durability * combination[id];
                sum.flavor += ingredient.flavor * combination[id];
                sum.texture += ingredient.texture * combination[id];
            }
            if sum.capacity > 0 && sum.durability > 0 && sum.flavor > 0 && sum.texture > 0 {
                let score = sum.capacity * sum.durability * sum.flavor * sum.texture;

                if max_score < score {
                    max_score = score;
                }
            }
        }
    }

    max_score
}

#[must_use]
pub fn solve_part_2(ingredients: &[Ingredient]) -> i64 {
    let mut max_score = 0;

    for combination in (0..ingredients.len())
        .map(|_| 0..=100)
        .multi_cartesian_product()
    {
        if combination.iter().sum::<i64>() == 100 {
            let mut sum = Ingredient {
                capacity: 0,
                durability: 0,
                flavor: 0,
                texture: 0,
                calories: 0,
            };

            for (id, ingredient) in ingredients.iter().enumerate() {
                sum.capacity += ingredient.capacity * combination[id];
                sum.durability += ingredient.durability * combination[id];
                sum.flavor += ingredient.flavor * combination[id];
                sum.texture += ingredient.texture * combination[id];
                sum.calories += ingredient.calories * combination[id];
            }
            if sum.capacity > 0
                && sum.durability > 0
                && sum.flavor > 0
                && sum.texture > 0
                && sum.calories == 500
            {
                let score = sum.capacity * sum.durability * sum.flavor * sum.texture;

                if max_score < score {
                    max_score = score;
                }
            }
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        );
        let input = parse_ingredients(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 62842880);
    }

    #[test]
    fn example_solve_part_2() {
        let input = String::from(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        );
        let input = parse_ingredients(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, 57600000);
    }
}
