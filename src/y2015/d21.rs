use std::{iter, ops::Add};

use itertools::Itertools;

#[derive(Clone)]
struct Entity {
    hp: u32,
    dmg: u32,
    armor: u32,
}

impl Entity {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        fn latter_u32(input: &str) -> u32 {
            input.split_once(": ").unwrap().1.parse().unwrap()
        }

        Self {
            hp: latter_u32(lines.next().unwrap()),
            dmg: latter_u32(lines.next().unwrap()),
            armor: latter_u32(lines.next().unwrap()),
        }
    }
}

#[derive(Clone, Debug)]
struct Equipment {
    cost: u32,
    dmg: u32,
    armor: u32,
}

impl Equipment {
    const fn new(cost: u32, dmg: u32, armor: u32) -> Self {
        Self { cost, dmg, armor }
    }
}

impl Add<Equipment> for Equipment {
    type Output = Equipment;

    fn add(self, rhs: Equipment) -> Self::Output {
        Equipment {
            cost: self.cost + rhs.cost,
            dmg: self.dmg + rhs.dmg,
            armor: self.armor + rhs.armor,
        }
    }
}

impl Add<&Equipment> for Equipment {
    type Output = Equipment;

    fn add(self, rhs: &Equipment) -> Self::Output {
        Equipment {
            cost: self.cost + rhs.cost,
            dmg: self.dmg + rhs.dmg,
            armor: self.armor + rhs.armor,
        }
    }
}

impl Add<&Equipment> for &Equipment {
    type Output = Equipment;

    fn add(self, rhs: &Equipment) -> Self::Output {
        Equipment {
            cost: self.cost + rhs.cost,
            dmg: self.dmg + rhs.dmg,
            armor: self.armor + rhs.armor,
        }
    }
}

impl Add<Equipment> for &Equipment {
    type Output = Equipment;

    fn add(self, rhs: Equipment) -> Self::Output {
        Equipment {
            cost: self.cost + rhs.cost,
            dmg: self.dmg + rhs.dmg,
            armor: self.armor + rhs.armor,
        }
    }
}

const WEAPONS: [Equipment; 5] = [
    Equipment::new(8, 4, 0),
    Equipment::new(10, 5, 0),
    Equipment::new(25, 6, 0),
    Equipment::new(40, 7, 0),
    Equipment::new(74, 8, 0),
];

const ARMOR: [Equipment; 6] = [
    Equipment::new(0, 0, 0),
    Equipment::new(13, 0, 1),
    Equipment::new(31, 0, 2),
    Equipment::new(53, 0, 3),
    Equipment::new(75, 0, 4),
    Equipment::new(102, 0, 5),
];

const RINGS: [Equipment; 7] = [
    Equipment::new(0, 0, 0),
    Equipment::new(25, 1, 0),
    Equipment::new(50, 2, 0),
    Equipment::new(100, 3, 0),
    Equipment::new(20, 0, 1),
    Equipment::new(40, 0, 2),
    Equipment::new(80, 0, 3),
];

fn simulate_battle(player: &Entity, boss: &Entity) -> bool {
    let player_turns = (boss.hp - 1) / (player.dmg - boss.armor.min(player.dmg - 1));
    let boss_turns = (player.hp - 1) / (boss.dmg - player.armor.min(boss.dmg - 1));
    player_turns <= boss_turns
}

pub fn solution_1(input: &str) -> String {
    let boss = Entity::from_str(input);
    RINGS
        .iter()
        .combinations(2)
        .map(|rings| rings[0] + rings[1])
        // No rings
        .chain(iter::once(Equipment::new(0, 0, 0)))
        .cartesian_product(ARMOR.iter())
        .cartesian_product(WEAPONS.iter())
        .filter_map(|((rings, armor), weapon)| {
            let e = rings + armor + weapon;
            if simulate_battle(
                &Entity {
                    hp: 100,
                    dmg: e.dmg,
                    armor: e.armor,
                },
                &boss,
            ) {
                Some(e.cost)
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let boss = Entity::from_str(input);
    RINGS
        .iter()
        .combinations(2)
        .map(|rings| rings[0] + rings[1])
        // No rings
        .chain(iter::once(Equipment::new(0, 0, 0)))
        .cartesian_product(ARMOR.iter())
        .cartesian_product(WEAPONS.iter())
        .filter_map(|((rings, armor), weapon)| {
            let e = rings + armor + weapon;
            if simulate_battle(
                &Entity {
                    hp: 100,
                    dmg: e.dmg,
                    armor: e.armor,
                },
                &boss,
            ) {
                None
            } else {
                Some(e.cost)
            }
        })
        .max()
        .unwrap()
        .to_string()
}
