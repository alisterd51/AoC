use std::collections::HashSet;

pub struct Replacement {
    from: String,
    to: String,
}

#[must_use]
pub fn parse_replacements(input: &str) -> (Vec<Replacement>, String) {
    let mut replacements = vec![];
    let mut molecule = String::new();
    let mut replacements_done = false;

    for line in input.lines() {
        if replacements_done {
            molecule = line.to_string();
        } else if line.is_empty() {
            replacements_done = true;
        } else if let Some((from, to)) = line.split_once(" => ") {
            replacements.push(Replacement {
                from: from.to_string(),
                to: to.to_string(),
            });
        }
    }

    (replacements, molecule)
}

fn revert_replacement(replacements: &[Replacement]) -> Vec<Replacement> {
    let mut new_replacements = vec![];

    for replacement in replacements {
        new_replacements.push(Replacement {
            from: replacement.to.clone(),
            to: replacement.from.clone(),
        });
    }

    new_replacements
}

fn generate_molecules_iter<'a>(
    replacements: &'a [Replacement],
    molecule: &'a str,
) -> impl Iterator<Item = String> + 'a {
    replacements.iter().flat_map(move |replacement| {
        molecule
            .match_indices(&replacement.from)
            .map(move |(index, _)| {
                format!(
                    "{}{}{}",
                    &molecule[..index],
                    &replacement.to,
                    &molecule[index + replacement.from.len()..]
                )
            })
    })
}

#[must_use]
pub fn solve_part_1(replacements: &[Replacement], molecule: &str) -> usize {
    let molecules: HashSet<String> = generate_molecules_iter(replacements, molecule).collect();
    molecules.len()
}

#[must_use]
pub fn solve_part_2(replacements: &[Replacement], final_molecule: &str) -> usize {
    let mut replacements = revert_replacement(replacements);
    replacements.sort_by_key(|b| std::cmp::Reverse(b.from.len()));
    let mut molecule = final_molecule.to_string();
    let mut steps = 0;

    while molecule != "e" {
        let new_molecule = generate_molecules_iter(&replacements, &molecule).next();
        if let Some(new_molecule) = new_molecule {
            molecule = new_molecule;
            steps += 1;
        } else {
            break;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from(
            "H => HO
H => OH
O => HH

HOH",
        );
        let (replacements, molecule) = parse_replacements(&input);
        let result = solve_part_1(&replacements, &molecule);
        assert_eq!(result, 4);
        let input = String::from(
            "H => HO
H => OH
O => HH

HOHOHO",
        );
        let (replacements, molecule) = parse_replacements(&input);
        let result = solve_part_1(&replacements, &molecule);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_solve_part_2() {
        let input = String::from(
            "e => H
e => O
H => HO
H => OH
O => HH

HOH",
        );
        let (replacements, molecule) = parse_replacements(&input);
        let result = solve_part_2(&replacements, &molecule);
        assert_eq!(result, 3);
        let input = String::from(
            "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO",
        );
        let (replacements, molecule) = parse_replacements(&input);
        let result = solve_part_2(&replacements, &molecule);
        assert_eq!(result, 6);
    }
}
