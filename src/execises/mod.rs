use crate::elves::ElvesTeam;
use crate::games::rock_paper_scissors::Guide;
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

