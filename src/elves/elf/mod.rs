use crate::{ items::{ backpack::Backpack, units::Calories }, tasks::{ Task, Section } };

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Elf {
    pub backpack: Backpack,
    pub capacity: Calories,
    pub tasks: Option<Task>,
}

impl Elf {
    pub fn new() -> Elf {
        Elf {
            backpack: Backpack::new(),
            capacity: 0,
            tasks: None,
        }
    }

    pub fn has_bigger_range(&self, elf: &Elf) -> bool {
        let (elf1_sec1, elf1_sec2) = self.tasks
            .as_ref()
            .expect("Elf 1 has no assigned tasks").sections;
        let (elf2_sec1, elf2_sec2) = elf.tasks
            .as_ref()
            .expect("Elf 2 has no assigned tasks").sections;

        (elf1_sec1..=elf1_sec2).contains(&elf2_sec1) && (elf1_sec1..=elf1_sec2).contains(&elf2_sec2)
    }
    pub fn has_smaller_range(&self, elf: &Elf) -> bool {
        let (elf1_sec1, elf1_sec2) = self.tasks
            .as_ref()
            .expect("Elf 1 has no assigned tasks").sections;
        let (elf2_sec1, elf2_sec2) = elf.tasks
            .as_ref()
            .expect("Elf 2 has no assigned tasks").sections;

        (elf2_sec1..=elf2_sec2).contains(&elf1_sec1) && (elf2_sec1..=elf2_sec2).contains(&elf1_sec2)
    }

    pub fn has_section(&self, elf: &Elf) -> bool {
        let (elf1_sec1, elf1_sec2) = self.tasks
            .as_ref()
            .expect("Elf 1 has no assigned tasks").sections;
        let (elf2_sec1, elf2_sec2) = elf.tasks
            .as_ref()
            .expect("Elf 2 has no assigned tasks").sections;

        (elf1_sec1..=elf1_sec2).contains(&elf2_sec1) ||
            (elf1_sec1..=elf1_sec2).contains(&elf2_sec2) ||
            (elf2_sec1..=elf2_sec2).contains(&elf1_sec1) ||
            (elf2_sec1..=elf2_sec2).contains(&elf1_sec2)
    }

    pub fn from_backpack(backpack: &Backpack) -> Elf {
        Elf { backpack: backpack.to_owned(), capacity: 0, tasks: None }
    }

    pub fn assign_tasks(&mut self, task_sections: &str) {
        let sections: Vec<_> = task_sections.split("-").collect();
        let start: Section = sections
            .first()
            .unwrap()
            .to_string()
            .parse()
            .expect("Unable to find task section");
        let end: Section = sections
            .last()
            .unwrap()
            .to_string()
            .parse()
            .expect("Unable to find task section");
        self.tasks = Some(Task::new(start, end));
    }

    pub fn set_capacity(&mut self) -> Self {
        self.capacity = self.backpack.items
            .iter()
            .map(|i| i.calories)
            .sum();
        self.clone()
    }
    pub fn get_capacity(&self) -> Calories {
        self.capacity
    }

    pub fn print_elf(&mut self) {
        let total_items = self.backpack.items.len();
        let total_calories = self.set_capacity().get_capacity();
        let mut calories: Vec<_> = self.backpack.items
            .iter()
            .map(|item| item.calories)
            .collect();
        calories.sort_by(|a, b| b.cmp(a));
        let highest = calories.first().unwrap();
        let smallest = calories.last().unwrap();

        print!(
            "++++++++++ Elf ++++++++++\nTotal Food Items: {}\nTotal Calories: {}\nSmallest Calorie: {}\nLargest Calorie: {}",
            total_items,
            total_calories,
            smallest,
            highest
        );
    }
}
