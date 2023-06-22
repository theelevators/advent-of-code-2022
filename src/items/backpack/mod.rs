use crate::{ items::Items, elves::elf::Elf };
use super::units::Priority;
use std::{ fs::read_to_string, str::Chars, collections::HashSet };
pub type Compartment = Vec<ItemType>;
pub type ItemList = String;
pub type PriorityList = Vec<ItemType>;
pub type Belongings = Vec<Backpack>;
pub type Cargo = Vec<ItemType>;


pub trait Travel {
    fn pack(&mut self, _file: &str)-> &Self {self}
}

pub trait Itemization {
    fn add_items(&mut self, _chars: Chars) {}
}
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ItemType {
    r#type: String,
    pub priority: Priority,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Backpack {
    pub items: Items,
    pub l_compartment: Compartment,
    pub r_compartment: Compartment,
}

impl Backpack {
    pub fn new() -> Backpack {
        Backpack {
            items: Items::new(),
            l_compartment: Compartment::new(),
            r_compartment: Compartment::new(),
        }
    }

    pub fn load_compartments(&mut self, list: ItemList) {
        let (l, r) = list.split_at(list.len() / 2);
        self.l_compartment.add_items(l.chars());
        self.r_compartment.add_items(r.chars());
    }

    pub fn get_priority_sum(self) -> Priority {
        let overlap: HashSet<_> = self.l_compartment
            .iter()
            .filter(|item| self.r_compartment.contains(item))
            .collect();
        overlap
            .iter()
            .map(|item| item.priority)
            .sum()
    }

    pub fn get_priority_items(&self) -> PriorityList {
        self.l_compartment
            .iter()
            .filter_map(|item| {
                match self.r_compartment.contains(item) {
                    true => Some(item.to_owned()),
                    false => None,
                }
            })
            .collect()
    }

    pub fn get_all_items(&mut self)->Cargo{
        let mut cargo = Cargo::new();
        cargo.append(&mut self.l_compartment.to_owned());
        cargo.append(&mut self.r_compartment.to_owned());
        cargo
    }
}

impl Itemization for Compartment {
    fn add_items(&mut self, chars: Chars) {
        for char in chars {
            if char.is_alphabetic() == false {
                panic!("Invalid Entry!")
            }
            match char.is_uppercase() {
                false => self.push(ItemType { r#type: char.to_string(), priority: 1 + (char as Priority) - ('a' as Priority) }),
                true => self.push(ItemType { r#type: char.to_string(), priority: 27 + (char as Priority) - ('A' as Priority) }),
            }
        }
    }
}

impl Travel for Belongings {
    fn pack(&mut self, file: &str)->&Self {
        for line in read_to_string(file).unwrap().lines() {
            let mut elf = Elf::new();
            elf.backpack.load_compartments(ItemList::from(line));
            self.push(elf.backpack);
        }
        self
    }
}
