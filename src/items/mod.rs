

use self::{units::Calories};
pub mod units;
pub mod backpack;
pub mod crates;
pub mod handheld;
 
pub type Instructions = Vec<Instruction>;
pub type Target = i32;
pub type Items = Vec<Item>;

pub type TagetSet = (Target, Target);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct  Instruction{
    seq: i32,
    cmd: Command,
    targets: TagetSet,
    qty: i32
}

impl Instruction {
    pub fn new(cmd: Command, targets: TagetSet, qty: i32, seq: i32) ->Instruction{
        Instruction{
            seq,
            cmd,
            targets,
            qty
        }
    }

}


#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]

pub struct Item {
   pub calories: Calories
}

impl Item {
    pub fn new(cal:Calories)->Item{
        Item { calories: cal }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Command {
    Move, 
    Stay
}

impl Command {
    fn new(cmd: &str)->Command{
        match cmd {
            "move" => Command::Move,
            _ => panic!("Invalid Command")
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Crane {
    CrateMover9001, 
    CrateMover9000,
    Generic
}
