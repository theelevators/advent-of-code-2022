use std::collections::HashMap;

use super::{ Instructions, Instruction, Command, Crane };

pub type Crates = String;
pub type Stacks = Vec<Stack>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ship {
    pub stacks: Stacks,
    instructions: Instructions,
    crane: Crane,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Stack {
    pub number: i32,
    pub crates: Crates,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            number: 0,
            crates: Crates::new(),
        }
    }
    pub fn take_crates(&mut self, qty: i32) -> Crates {
        let mut taken = Crates::new();
        for _ in 0..qty {
            taken.push(self.crates.chars().to_owned().next().unwrap());
            self.crates.remove(0);
        }
        taken
    }
    pub fn receive_bulk_crates(&mut self, crates: Crates) {
        self.crates.insert_str(0, crates.as_str());
    }
    pub fn receive_crates(&mut self, crates: Crates) {
        for c in crates.chars() {
            self.crates.insert(0, c);
        }
    }
    pub fn take_bulk_crates(&mut self, qty: i32) -> Crates {
        let chars: Vec<_> = self.crates
            .chars()
            .enumerate()
            .filter(|(idx, ..)| qty > (idx.to_owned() as i32))
            .collect();
        let chars: Vec<_> = chars
            .iter()
            .map(|x| String::from(x.1))
            .collect();

        for _ in 0..qty {
            self.crates.remove(0);
        }

        Crates::from(chars[..qty as usize].to_vec().join(""))
    }
}

impl Ship {
    pub fn new() -> Ship {
        Ship { stacks: Stacks::new(), instructions: Instructions::new(), crane: Crane::Generic }
    }

    pub fn set_crane(&mut self, crane: Crane) {
        self.crane = crane;
    }

    pub fn load_stacks(&mut self, cargo: &String) {
        let mut map: HashMap<i32, Stack> = HashMap::new();
        for line in cargo.lines() {
            for (idx, c) in line.chars().enumerate() {
                if c.is_alphabetic() {
                    map.entry(idx as i32)
                        .and_modify(|s| s.crates.push(c))
                        .or_insert(Stack { number: 0, crates: Crates::from(c) });
                }
                if c.is_numeric() {
                    map.entry(idx as i32)
                        .and_modify(|s| {
                            s.number = c.to_digit(10).unwrap() as i32;
                        })
                        .or_insert(Stack { number: 0, crates: Crates::from(c) });
                }
            }
        }
        self.stacks = map
            .values()
            .map(|v| v.to_owned())
            .collect();
    }

    pub fn move_bulk(&mut self, from: i32, to: i32, qty: i32) {
        let giver = self.stacks
            .iter_mut()
            .find(|stack| stack.number == from)
            .expect("From Stack not found");
        let taken = giver.take_bulk_crates(qty);
        let reciever = self.stacks
            .iter_mut()
            .find(|stack| stack.number == to)
            .expect("To Stack not found!");
        reciever.receive_bulk_crates(taken);
    }

    pub fn move_load(&mut self, from: i32, to: i32, qty: i32) {
        let giver = self.stacks
            .iter_mut()
            .find(|stack| stack.number == from)
            .expect("From Stack not found");
        let taken = giver.take_crates(qty);
        let reciever = self.stacks
            .iter_mut()
            .find(|stack| stack.number == to)
            .expect("To Stack not found!");
        reciever.receive_crates(taken);
    }

    pub fn set_instructions(&mut self, file: &String) {
        let mut count = 0;
        for line in file.lines() {
            match line {
                "" => {
                    break;
                }
                _ => {
                    let cols: Vec<_> = line
                        .split_whitespace()
                        .filter(|c| !c.is_empty())
                        .collect();
                    let mut inst = Instruction::new(Command::Stay, (0, 0), 0, count);
                    for chunk in cols.chunks(2) {
                        match chunk[0] {
                            "move" => {
                                inst.cmd = Command::Move;
                                inst.qty = chunk[1].parse().expect("Unable to read qty");
                            }
                            "from" => {
                                inst.targets.0 = chunk[1]
                                    .parse()
                                    .expect("Unable to read from number");
                            }
                            "to" => {
                                inst.targets.1 = chunk[1]
                                    .parse()
                                    .expect("Unable to read to number");
                            }
                            _ => println!("Invalid instructions: {}", chunk[0]),
                        }
                    }
                    self.instructions.insert(count as usize, inst);
                }
            }
            count += 1;
        }
    }

    pub fn operate_crane(&mut self, instruction: &Instruction) {
        match self.crane {
            Crane::CrateMover9000 => {
                match instruction.cmd {
                    Command::Move =>
                        self.move_load(
                            instruction.targets.0,
                            instruction.targets.1,
                            instruction.qty
                        ),
                    _ => panic!("Command not supported"),
                }
            }
            Crane::CrateMover9001 => {
                match instruction.cmd {
                    Command::Move =>
                        self.move_bulk(
                            instruction.targets.0,
                            instruction.targets.1,
                            instruction.qty
                        ),
                    _ => panic!("Command not supported"),
                }
            }
            _ => panic!("Generic Cranes are unsafe!!!!"),
        }
    }

    pub fn follow_instructions(&mut self) {
        self.instructions
            .clone()
            .iter()
            .for_each(|inst| self.operate_crane(inst));
    }

    pub fn get_top_crates(&mut self) -> Crates {
        self.stacks.sort();
        let c = self.stacks
            .iter()
            .map(|stack|
                Crates::from(
                    stack.crates.chars().to_owned().next().expect("Unable to collect top crates")
                )
            )
            .collect::<Vec<String>>();

        c.join("")
    }
}
