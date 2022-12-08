// An elf was tasked with putting supplies in rucksacks for the next venture into the jungle
// each rucksack has 2 compartments there is to be no duplicates between the compartments
//items are represented by single characters a-Z caps matter.  
// each item is given a priority score a=1 Z=52
// The elf messed up and put duplicates in the compartments.  
//Find the duplicates and calculate the priority sum.

fn main() {
    const VALID_ITEMS: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 
        'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 
        'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
    // let mut shared_items = Vec::new();
    let mut priority_sum: u32 = 0;
    let mut group_item_sum: u32 = 0;
    let all_rucksacks: Vec<&str> = include_str!("input.txt").lines().collect(); 


    fn calc_priority(item: &char) -> u32{
        let priority: u32 = VALID_ITEMS.iter().position(|i| i == item).unwrap() as u32;
        return priority + 1;
    }
    
    
    for rucksack in &all_rucksacks{
        let split_index: usize = (rucksack.len())/2;
        
        let compartment1:&str = &rucksack[0..split_index];
        let compartment2:&str = &rucksack[split_index..];
        let mut shared_items:Vec<char> = vec![];

        for item in rucksack.chars(){
            if compartment1.contains(item) && compartment2.contains(item){
                if shared_items.contains(&item){
                }
                else {
                    shared_items.push(item);
                    priority_sum += calc_priority(&item);
                }
            }
        }
    }

    for sack_group in all_rucksacks.chunks(3){
        let a = sack_group[0];
        let b = sack_group[1];
        let c = sack_group[2];


        for item in a.chars(){
            if b.contains(item) && c.contains(item){
                group_item_sum += calc_priority(&item);
                break;
            }
        }
    }

    println!("Part 1 priority sum: {}", priority_sum);
    println!("Part 2 group_sum: {}", group_item_sum)

}

