pub mod elf;
use crate::{items::{
    Item,
    units::{ Calories, Priority },
    backpack::{ Belongings, ItemType },
}, tasks::{CleanUpCrew, CleanUp}};
use std::fs::read_to_string;
use elf::Elf;

pub type Elves = Vec<Elf>;

#[derive(Clone, Debug)]
pub struct ElvesTeam {
    pub elves: Elves,
    pub team_badge: Option<ItemType>,
}

impl ElvesTeam {
    pub fn new() -> ElvesTeam {
        ElvesTeam {
            elves: Elves::new(),
            team_badge: None,
        }
    }

    pub fn from_team(team: &[Elf]) -> ElvesTeam {
        ElvesTeam { elves: Elves::from(team), team_badge: None }
    }

    pub fn new_team_w_cal(filename: &str) -> ElvesTeam {
        ElvesTeam {
            elves: Elves::new_team_w_cal(filename),
            team_badge: None,
        }
    }

    pub fn new_team_w_cleanup_list(filename: &str)->ElvesTeam{
        let mut elves = Elves::new();
        for line in read_to_string(filename).unwrap().lines() {
            let (elf1, elf2) = CleanUpCrew::new_crew().assign_tasks(line);
            elves.push(elf1);
            elves.push(elf2);
        }
        ElvesTeam {
            elves,
            team_badge: None,
        }
    }

    pub fn count_tasks_overlaps(filename: &str) ->i32{
        let mut count = 0;

        for line in read_to_string(filename).unwrap().lines() {
            match CleanUpCrew::new_crew().assign_tasks(line).overlaps() {
                true => count += 1,
                false => ()
            }
        }
  
        count
    }

    pub fn count_all_tasks_overlaps(filename: &str) ->i32{
        let mut count = 0;

        for line in read_to_string(filename).unwrap().lines() {
            match CleanUpCrew::new_crew().assign_tasks(line).all_overlaps() {
                true => count += 1,
                false => ()
            }
        }
  
        count
    }

    pub fn rank_team(&mut self) -> Self {
        self.elves.sort_by(|a, b| b.capacity.cmp(&a.capacity));

        self.clone()
    }

    pub fn get_team_capacity(&self) -> Calories {
        self.elves
            .clone()
            .into_iter()
            .map(|elf| elf.capacity)
            .sum()
    }

    pub fn get_team_priority(&self) -> Priority {
        self.elves
            .iter()
            .map(|elf| elf.backpack.clone().get_priority_sum())
            .sum()
    }
    pub fn from_belongings(luggage: Belongings) -> ElvesTeam {
        ElvesTeam {
            elves: luggage
                .iter()
                .map(|backpack| Elf::from_backpack(backpack))
                .collect(),
            team_badge: None,
        }
    }
    pub fn set_team_badge(&mut self) {
        self.team_badge = self.elves[0].backpack
            .get_all_items()
            .iter()
            .find(|item|
                self.elves.iter().all(|elf| elf.backpack.to_owned().get_all_items().contains(item))
            )
            .cloned();
    }

    pub fn sum_group_by_priority(&self) -> Priority {
        let mut priority_sum = 0;

        for team in self.elves.chunks(3) {
            let mut mini_team: ElvesTeam = ElvesTeam::from_team(team);
            mini_team.set_team_badge();

            priority_sum += mini_team.team_badge.expect(
                "Not all elves have same badge."
            ).priority;
        }
        priority_sum
    }
}

pub trait TeamWork {
    fn baller_elf(&mut self) -> Calories;
    fn new_team_w_cal(file: &str) -> Elves;
}

impl TeamWork for Elves {
    fn baller_elf(&mut self) -> Calories {
        let mut highest: Calories = 0;

        for elf in self {
            let current: Calories = elf.set_capacity().get_capacity();
            if current > highest {
                highest = current;
            }
        }
        return highest;
    }

    fn new_team_w_cal(filename: &str) -> Elves {
        let mut team = Elves::new();
        let mut elf = Elf::new();
        for line in read_to_string(filename).unwrap().lines() {
            if line == "" {
                team.push(elf.clone().set_capacity());
                elf = Elf::new();
                continue;
            }
            let calories: Calories = line
                .parse::<Calories>()
                .expect("Unable to read numeric line.");
            elf.backpack.items.push(Item::new(calories));
        }

        team
    }
}
