#[derive(Default)]
pub struct Sue {
    pub id: u64,
    pub children: Option<u64>,
    pub cats: Option<u64>,
    pub samoyeds: Option<u64>,
    pub pomeranians: Option<u64>,
    pub akitas: Option<u64>,
    pub vizslas: Option<u64>,
    pub goldfish: Option<u64>,
    pub trees: Option<u64>,
    pub cars: Option<u64>,
    pub perfumes: Option<u64>,
}

#[must_use]
pub fn parse_sues(input: &str) -> Vec<Sue> {
    let mut sues = vec![];

    for line in input.lines() {
        if let Some((sue, values)) = line.split_once(':')
            && let Some((_, id)) = sue.split_once(' ')
            && let Ok(id) = id.parse::<u64>()
        {
            let mut sue = Sue {
                id,
                ..Default::default()
            };
            for value in values.split(',') {
                if let Some((name, value)) = value.split_once(':')
                    && let Ok(value) = value.trim().parse::<u64>()
                {
                    match name.trim() {
                        "children" => sue.children = Some(value),
                        "cats" => sue.cats = Some(value),
                        "samoyeds" => sue.samoyeds = Some(value),
                        "pomeranians" => sue.pomeranians = Some(value),
                        "akitas" => sue.akitas = Some(value),
                        "vizslas" => sue.vizslas = Some(value),
                        "goldfish" => sue.goldfish = Some(value),
                        "trees" => sue.trees = Some(value),
                        "cars" => sue.cars = Some(value),
                        "perfumes" => sue.perfumes = Some(value),
                        _ => {}
                    }
                }
            }
            sues.push(sue);
        }
    }

    sues
}

#[must_use]
pub fn solve_part_1(sues: &[Sue], searched_sue: &Sue) -> u64 {
    for sue in sues {
        if (sue.children.is_some() && sue.children != searched_sue.children)
            || (sue.cats.is_some() && sue.cats != searched_sue.cats)
            || (sue.samoyeds.is_some() && sue.samoyeds != searched_sue.samoyeds)
            || (sue.pomeranians.is_some() && sue.pomeranians != searched_sue.pomeranians)
            || (sue.akitas.is_some() && sue.akitas != searched_sue.akitas)
            || (sue.vizslas.is_some() && sue.vizslas != searched_sue.vizslas)
            || (sue.goldfish.is_some() && sue.goldfish != searched_sue.goldfish)
            || (sue.trees.is_some() && sue.trees != searched_sue.trees)
            || (sue.cars.is_some() && sue.cars != searched_sue.cars)
            || (sue.perfumes.is_some() && sue.perfumes != searched_sue.perfumes)
        {
            continue;
        }
        return sue.id;
    }
    0
}

#[must_use]
pub fn solve_part_2(sues: &[Sue], searched_sue: &Sue) -> u64 {
    for sue in sues {
        if (sue.children.is_some() && sue.children != searched_sue.children)
            || (sue.cats.is_some() && sue.cats <= searched_sue.cats)
            || (sue.samoyeds.is_some() && sue.samoyeds != searched_sue.samoyeds)
            || (sue.pomeranians.is_some() && sue.pomeranians >= searched_sue.pomeranians)
            || (sue.akitas.is_some() && sue.akitas != searched_sue.akitas)
            || (sue.vizslas.is_some() && sue.vizslas != searched_sue.vizslas)
            || (sue.goldfish.is_some() && sue.goldfish >= searched_sue.goldfish)
            || (sue.trees.is_some() && sue.trees <= searched_sue.trees)
            || (sue.cars.is_some() && sue.cars != searched_sue.cars)
            || (sue.perfumes.is_some() && sue.perfumes != searched_sue.perfumes)
        {
            continue;
        }
        return sue.id;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from(
            "Sue 41: children: 2, cars: 2
Sue 42: children: 3, cars: 2",
        );
        let sues = parse_sues(&input);
        let searched_sue = Sue {
            id: 0,
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1),
        };
        let result = solve_part_1(&sues, &searched_sue);
        assert_eq!(result, 42);
    }

    #[test]
    fn example_solve_part_2() {}
}
