
// I know its gross I haven't learned vectors fully yet.  
//I will rewrite with map in an rewrite.rs file
fn main() {
    let mut cal_list:Vec<u64> = vec![];

    let elves = include_str!("input.txt").split("\n\n");
    for elf in elves {
        let snacks = elf.split("\n");
        let mut total_calories: u64 = 0;
        for food in snacks {
            total_calories += food.parse::<u64>().unwrap()
        }
        cal_list.push(total_calories)
        
    }
    cal_list.sort();
    cal_list.reverse();
    println!("{}", cal_list[0]);
}


