use crate::elves::elf::Elf;

pub type CleanUpCrew = (Elf, Elf);
pub type Section = i32;

pub trait CleanUp {
    fn assign_tasks(&mut self, task_list: &str) -> Self;
    fn overlaps(&self) -> bool;
    fn new_crew() -> CleanUpCrew;
    fn all_overlaps(&self) -> bool;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Task {
    pub sections: (Section, Section),
}

impl Task {
    pub fn new(start: Section, end: Section) -> Task {
        Task { sections: (start, end) }
    }
}

impl CleanUp for CleanUpCrew {
    fn assign_tasks(&mut self, task_list: &str) -> Self {
        let list: Vec<_> = task_list
            .split(",")
            .filter(|s| s.contains("-"))
            .collect();

        self.0.assign_tasks(list.first().expect("Unable to read task list"));
        self.1.assign_tasks(list.last().expect("Unable to read task list"));
        self.to_owned()
    }
    fn new_crew() -> CleanUpCrew {
        (Elf::new(), Elf::new())
    }
    fn overlaps(&self) -> bool {
        self.0.has_bigger_range(&self.1) ||
            self.0.has_smaller_range(&self.1) ||
            self.0.tasks == self.1.tasks
    }

    fn all_overlaps(&self) -> bool {
        self.0.has_section(&self.1) ||
            self.0.tasks == self.1.tasks
    }
}
