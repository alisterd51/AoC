pub struct Reindeer {
    flying_speed: u64,
    flying_time: u64,
    resting_time: u64,
}

#[must_use]
pub fn parse_reindeer(input: &str) -> Vec<Reindeer> {
    let mut reindeer = vec![];

    for line in input.lines() {
        if let Some(line) = line.strip_suffix(".")
            && let words = line.split_whitespace().collect::<Vec<&str>>()
            && words.len() == 15
            && let Ok(flying_speed) = words[3].parse::<u64>()
            && let Ok(flying_time) = words[6].parse::<u64>()
            && let Ok(resting_time) = words[13].parse::<u64>()
        {
            reindeer.push(Reindeer {
                flying_speed,
                flying_time,
                resting_time,
            });
        }
    }

    reindeer
}

#[must_use]
pub fn solve_part_1(reindeer: &[Reindeer], time: u64) -> u64 {
    let mut max_distance = 0;
    for reindeer in reindeer {
        let complete_cycle = time / (reindeer.flying_time + reindeer.resting_time);
        let time = time % (reindeer.flying_time + reindeer.resting_time);
        let distance = (complete_cycle * reindeer.flying_time + reindeer.flying_time.min(time))
            * reindeer.flying_speed;
        if max_distance < distance {
            max_distance = distance;
        }
    }

    max_distance
}

#[must_use]
pub fn solve_part_2(reindeer: &[Reindeer], time: u64) -> Option<u64> {
    let mut points = vec![0u64; reindeer.len()];
    let mut distances = vec![0u64; reindeer.len()];
    for time in 0..time {
        for (index, reindeer) in reindeer.iter().enumerate() {
            let time = time % (reindeer.flying_time + reindeer.resting_time);
            if time < reindeer.flying_time {
                distances[index] += reindeer.flying_speed;
            }
        }
        if let Some(distance_max) = distances.iter().max() {
            for (index, distance) in distances.iter().enumerate() {
                if distance == distance_max {
                    points[index] += 1;
                }
            }
        }
    }

    points.iter().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        );
        let input = parse_reindeer(&input);
        let result = solve_part_1(&input, 1000);
        assert_eq!(result, 1120);
    }

    #[test]
    fn example_solve_part_2() {
        let input = String::from(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        );
        let input = parse_reindeer(&input);
        let result = solve_part_2(&input, 1000);
        assert_eq!(result, Some(689));
    }
}
