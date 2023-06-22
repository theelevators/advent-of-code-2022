mod tasks;
mod elves;
mod games;
mod items;
mod execises;
use execises::{ elves_calories, elves_tournament, rucksack_reorganization, camp_cleanup, supply_stacks, tuning_trouble };


fn main() {
    elves_calories();
    elves_tournament();
    rucksack_reorganization();
    camp_cleanup() ;
    supply_stacks();
    tuning_trouble();

    
}
