// A = X = Rock = 1pts
// B = Y = Paper = 2pts
// C = Z = Scissors =3pts
// lose = 0pts
// draw = 3pts
// win = 6pts

fn main() {
    //creates a vector of tuples, where the tuple[0] = their move and tuple[1] is my move
    let _games: Vec<(&str, &str)> = include_str!("input.txt")
        .split("\n")
        .map(|game| game.split_once(" ").unwrap())
        .collect();
    
    let total_gesture_score: u64 = _games.iter()
        .map(|game| calc_gesture_score(game))
        .collect::<Vec<u64>>()
        .iter()
        .sum();
    
    let total_game_condition_score: u64 = _games.iter()
        .map(|game| calc_game_condition_score(game))
        .collect::<Vec<u64>>()
        .iter()
        .sum();
    
    let total_score : u64 = total_gesture_score + total_game_condition_score;

    println!("Total Gesture Score: {:?}", total_gesture_score);
    println!("Total Condition Score: {:?}", total_game_condition_score);
    println!("Total Score: {:}", total_score);
}

fn calc_gesture_score(game: &(&str, &str)) -> u64 {
    let  my_move = game.1;
    match my_move {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn calc_game_condition_score (game: &(&str, &str)) -> u64 {
    let opponent_move = game.0;
    let my_move = game.1;
    match opponent_move {
        "A" => 
            match my_move {
                "X" => 3,
                "Y" => 6,
                "Z" => 0,
                _ => 0,
            },
        "B" => 
            match my_move {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            },
        "C" => 
            match my_move {
                "X" => 6,
                "Y" => 0,
                "Z" => 3,
                _ => 0,
            }
        _ => 0,
    }
}
