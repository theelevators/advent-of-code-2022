mod tasks;
mod elves;
mod games;
mod items;
mod execises;
use std::fs::read_to_string;

use execises::{ elves_calories, elves_tournament, rucksack_reorganization, camp_cleanup };
use items::crates::Ship;

fn main() {
    // elves_calories();
    // elves_tournament();
    // rucksack_reorganization();
    // camp_cleanup() ;

    let file = read_to_string("stacks.txt").expect("File not found!");
    let mut parts = file.split("\r\n\r\n");

    let load = parts.next().expect("Unable to make map!").split("").collect();

    let instructions = parts.next().expect("Unable to read instrcutions!").split("").collect();

    let mut ship = Ship::new().load_stacks(&load);

    ship.set_instructions(&instructions);

    ship = ship.run_instruction_set();

    println!("{:?}", ship.get_top_crates());
}
