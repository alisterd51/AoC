use itertools::{Itertools, iproduct};
use std::iter::once;

#[derive(Copy, Clone)]
struct Item {
    #[allow(dead_code)]
    name: &'static str,
    cost: u64,
    damage: u64,
    armor: u64,
}

const WEAPONS: &[Item] = &[
    Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    },
];
const ARMORS: &[Item] = &[
    Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    },
];
const RINGS: &[Item] = &[
    Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

pub struct Stats {
    hit_points: u64,
    damage: u64,
    armor: u64,
}

#[must_use]
pub fn parse_stats(input: &str) -> Stats {
    let mut stats = Stats {
        hit_points: 0,
        damage: 0,
        armor: 0,
    };

    for line in input.lines() {
        if let Some((stat_name, value)) = line.split_once(": ")
            && let Ok(value) = value.parse::<u64>()
        {
            match stat_name {
                "Hit Points" => stats.hit_points = value,
                "Damage" => stats.damage = value,
                "Armor" => stats.armor = value,
                _ => {}
            }
        }
    }

    stats
}

fn fight(player: &Stats, boss: &Stats) -> bool {
    let player_damage = player.damage.saturating_sub(boss.armor).max(1);
    let boss_damage = boss.damage.saturating_sub(player.armor).max(1);
    let turns_to_kill_boss = boss.hit_points.div_ceil(player_damage);
    let turns_to_kill_player = player.hit_points.div_ceil(boss_damage);

    turns_to_kill_boss <= turns_to_kill_player
}

#[must_use]
pub fn solve_part_1(boss: &Stats) -> u64 {
    let armor_choices: Vec<Option<Item>> =
        once(None).chain(ARMORS.iter().copied().map(Some)).collect();
    let mut ring_choices: Vec<Vec<Item>> = vec![vec![]];
    ring_choices.extend(RINGS.iter().copied().combinations(1));
    ring_choices.extend(RINGS.iter().copied().combinations(2));
    let mut min_cost = u64::MAX;

    for (weapon, armor, rings) in iproduct!(WEAPONS.iter().copied(), armor_choices, ring_choices) {
        let mut player = Stats {
            hit_points: 100,
            damage: 0,
            armor: 0,
        };
        let mut cost = 0;

        cost += weapon.cost;
        player.damage += weapon.damage;
        player.armor += weapon.armor;
        if let Some(armor) = armor {
            cost += armor.cost;
            player.damage += armor.damage;
            player.armor += armor.armor;
        }
        for ring in rings {
            cost += ring.cost;
            player.damage += ring.damage;
            player.armor += ring.armor;
        }
        if cost < min_cost && fight(&player, boss) {
            min_cost = cost;
        }
    }

    min_cost
}

#[must_use]
pub fn solve_part_2(boss: &Stats) -> u64 {
    let armor_choices: Vec<Option<Item>> =
        once(None).chain(ARMORS.iter().copied().map(Some)).collect();
    let mut ring_choices: Vec<Vec<Item>> = vec![vec![]];
    ring_choices.extend(RINGS.iter().copied().combinations(1));
    ring_choices.extend(RINGS.iter().copied().combinations(2));
    let mut max_cost = u64::MIN;

    for (weapon, armor, rings) in iproduct!(WEAPONS.iter().copied(), armor_choices, ring_choices) {
        let mut player = Stats {
            hit_points: 100,
            damage: 0,
            armor: 0,
        };
        let mut cost = 0;

        cost += weapon.cost;
        player.damage += weapon.damage;
        player.armor += weapon.armor;
        if let Some(armor) = armor {
            cost += armor.cost;
            player.damage += armor.damage;
            player.armor += armor.armor;
        }
        for ring in rings {
            cost += ring.cost;
            player.damage += ring.damage;
            player.armor += ring.armor;
        }
        if cost > max_cost && !fight(&player, boss) {
            max_cost = cost;
        }
    }

    max_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let player = String::from(
            "Hit Points: 8
Damage: 5
Armor: 5",
        );
        let boss = String::from(
            "Hit Points: 12
Damage: 7
Armor: 2",
        );
        let player = parse_stats(&player);
        let boss = parse_stats(&boss);
        let result = fight(&player, &boss);
        assert!(result);
        let player = String::from(
            "Hit Points: 8
Damage: 5
Armor: 5",
        );
        let boss = String::from(
            "Hit Points: 15
Damage: 7
Armor: 2",
        );
        let player = parse_stats(&player);
        let boss = parse_stats(&boss);
        let result = fight(&player, &boss);
        assert!(!result);
    }

    #[test]
    fn example_solve_part_2() {}
}
