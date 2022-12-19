use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Material {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Self::Ore),
            "clay" => Ok(Self::Clay),
            "obsidian" => Ok(Self::Obsidian),
            "geode" => Ok(Self::Geode),
            _ => Err(()),
        }
    }
}

impl Material {
    fn all() -> [Material; 4] {
        [
            Material::Ore,
            Material::Clay,
            Material::Obsidian,
            Material::Geode,
        ]
    }

    fn index(&self) -> usize {
        match self {
            Material::Ore => 0,
            Material::Clay => 1,
            Material::Obsidian => 2,
            Material::Geode => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    id: usize,
    robots_costs: [[usize; 3]; 4],
}

pub fn generator(input: &str) -> Vec<Blueprint> {
    let pattern = regex::Regex::new(r#"Each (\w+) robot costs (.*?)\."#).unwrap();
    input
        .split("Blueprint ")
        .filter_map(|text| {
            let (blueprint_id, blueprint_text) = text.split_once(':')?;
            let mut robots_costs = [[0usize; 3]; 4];
            for captures in pattern.captures_iter(blueprint_text) {
                let robot_type: Material = captures[1].parse().unwrap();
                let costs = &mut robots_costs[robot_type.index()];
                for cost_description in captures[2].split(" and ") {
                    let (num, material) = cost_description.split_once(' ').unwrap();
                    let num: usize = num.parse().unwrap();
                    let material: Material = material.parse().unwrap();
                    costs[material.index()] = num;
                }
            }
            Some(Blueprint {
                id: blueprint_id.parse().unwrap(),
                robots_costs,
            })
        })
        .collect()
}

#[derive(Debug, Clone)]
struct State {
    robots: [usize; 4],
    ores: [usize; 4],
}

impl Default for State {
    fn default() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            ores: [0, 0, 0, 0],
        }
    }
}

impl State {
    fn most_geode(&self, blueprint: &Blueprint, remainint_time: usize) -> usize {
        let mut best = self.clone().wait_minutes(remainint_time).ores[Material::Geode.index()];
        for material in Material::all().into_iter() {
            let Some(time_required) = self.time_to_build(blueprint, material) else { break; };
            let time_after_built = time_required + 1;
            if time_after_built < remainint_time {
                let after_waiting_and_building = self
                    .clone()
                    .wait_minutes(time_after_built)
                    .build_robot(blueprint, material);
                best = best.max(
                    after_waiting_and_building
                        .most_geode(blueprint, remainint_time - time_after_built),
                );
            }
        }
        best
    }

    fn time_to_build(&self, blueprint: &Blueprint, material: Material) -> Option<usize> {
        let mut max_required = 0;
        for (cost, (ore, num_robots)) in blueprint.robots_costs[material.index()]
            .iter()
            .zip(self.ores.iter().zip(self.robots))
        {
            if ore < cost {
                if num_robots == 0 {
                    return None;
                }
                let extra_ore_required = cost - ore;
                let mut time_required = extra_ore_required / num_robots;
                if 0 < extra_ore_required % num_robots {
                    time_required += 1;
                }
                max_required = max_required.max(time_required);
            }
        }
        Some(max_required)
    }

    fn wait_minutes(mut self, minutes: usize) -> Self {
        for (ore, num_robots) in self.ores.iter_mut().zip(self.robots) {
            *ore += num_robots * minutes;
        }
        self
    }

    fn build_robot(mut self, blueprint: &Blueprint, material: Material) -> Self {
        let costs = &blueprint.robots_costs[material.index()];
        for (ore, cost) in self.ores.iter_mut().zip(costs) {
            *ore -= *cost;
        }
        self.robots[material.index()] += 1;
        self
    }
}

pub fn part_1(input: &[Blueprint]) -> usize {
    input
        .iter()
        .map(|blueprint| blueprint.id * State::default().most_geode(blueprint, 24))
        .sum()
}

pub fn part_2(input: &[Blueprint]) -> usize {
    input
        .iter()
        .take(3)
        .map(|blueprint| State::default().most_geode(blueprint, 32))
        .product()
}
