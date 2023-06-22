use std::fs::read_to_string;

use crate::elves::ElvesTeam;
use crate::games::rock_paper_scissors::Guide;
use crate::items::Crane;
use crate::items::crates::Ship;
use crate::items::handheld::Handheld;
use crate::items::units::Calories;
use crate::items::{ backpack::{ Belongings, Travel }, units::Priority };

pub fn elves_calories() {
    let file_name: &str = "elf_food.txt";
    let mut team: ElvesTeam = ElvesTeam::new_team_w_cal(file_name);
    let mut top3:Calories = 0;
    team = team.rank_team();

    for i in 0..3 {
        top3 += team.elves[i].capacity;
    }
    println!("\n######### Day 1 #########\n");
    println!("++++++++++ Results ++++++++++");
    println!("\nThe largest pack is: {} \nThe top 3 combined is: {}", team.elves[0].capacity, top3);
    println!("\n----- Heaviest Pack Weight -----\n");
    team.elves.first().unwrap().to_owned().print_elf();
    println!("\n\n----- Lightest Pack Weight -----\n");
    team.elves.last().unwrap().to_owned().print_elf();
}

pub fn elves_tournament() {
    let mut t_guide: Guide = Guide::new("guide.txt");
    t_guide = t_guide.new_predictions("guide.txt");
    println!("\n######### Day 2 #########\n");
    println!("Points if I follow my guide: {}\n", t_guide.to_owned().follow_my_guide());
    println!("Points if I follow elfs guide: {}\n", t_guide.follow_elfs_guide());
}

pub fn rucksack_reorganization() {
    let luggage: Belongings = Belongings::new().pack("rucksack.txt").to_owned();
    let team_priority:Priority = ElvesTeam::from_belongings(luggage.to_owned()).sum_group_by_priority();
    let total_priority: Priority = luggage
        .iter()
        .map(|backpack| backpack.to_owned().get_priority_sum())
        .sum();
    println!("\n######### Day 3 #########\n");
    println!("Priority by teams: {team_priority}\n Total Priority: {total_priority}\n")
}

pub fn camp_cleanup(){
    
    let file: &str = "cleanup.txt";
    println!("\n######### Day 4 #########\n");
    println!("\nThe elves overlap count is: {}", ElvesTeam::count_tasks_overlaps(file));
    println!("\nThe elves total section overlap count is: {}\n", ElvesTeam::count_all_tasks_overlaps(file));

}

pub fn supply_stacks(){
    
    let file = read_to_string("stacks.txt").expect("File not found!");
    let mut parts = file.split("\r\n\r\n");
    let load = parts.next().expect("Unable to make map!").split("").collect();
    let instructions = parts.next().expect("Unable to read instrcutions!").split("").collect();

    let mut ship = Ship::new();
    ship.load_stacks(&load);
    ship.set_crane(Crane::CrateMover9000);
    ship.set_instructions(&instructions);
    ship.follow_instructions();

    println!("\n######### Day 5 #########\n");
    println!("With CrateMover 9000 you get: {:?}", ship.get_top_crates());
    ship.set_crane(Crane::CrateMover9001);
    ship.load_stacks(&load);
    ship.follow_instructions();
    println!("With CrateMover 9001 you get: {:?}", ship.get_top_crates());

}


pub fn tuning_trouble(){
    let file: String = read_to_string("tuningtrouble.txt").expect("File not found!");
    let mut handheld: Handheld = Handheld::new();

    println!("\n######### Day 6 #########\n");
    handheld.set_frequency(4);
    handheld.read_signals(&file);
    handheld.set_frequency(14);
    handheld.read_signals(&file);
}