//Part 1, calculate the total points from the game plan given in input.txt
// Opponent moves encoded A,B,C.  Strategy moves X,Y,Z
// You get points for the gesture you use and if you win or lose.
// A = X = Rock = 1pts
// B = Y = Paper = 2pts
// C = Z = Scissors =3pts
// lose = 0pts
// draw = 3pts
// win = 6pts

//Part 2
// New strategy given, X,Y,Z represent win conditions not gestures, need to recalculate total points
// X = need to lose
// Y = need to draw
// Z = need to win

enum End_Game_Condition {
    Win,
    Loss,
    Draw
}

fn main() {
    const GESTURES : [&str;3] = ["A", "B", "C"];
    const ENCODED_WL : [[&str; 3];3] = [["B","A","C"],["C","B","A"], ["A","C","B"]];
    const ROCK_WL_ENCODING: [&str; 3]  = ["C","A","B"];
    const PAPER_WL_ENCODING: [&str; 3] = ["A","B","C"];
    const SCISSORS_WL_ENCODING: [&str; 3] = ["B","C","A"];

    fn calc_wld_total(game_list: &Vec<(&str, &str)>) -> u64 {
        let total: u64 = game_list.iter()
        .map(|game|(calc_wld_score(game)))
        .collect::<Vec<u64>>()
        .iter()
        .sum();
    
        return total;
    }
    
    fn calc_wld_score(game: &(&str, &str)) -> u64{
        match game.1 {
            "A" => compare_gestures(game.0, ROCK_WL_ENCODING),
            "B" => compare_gestures(game.0, PAPER_WL_ENCODING),
            "C" => compare_gestures(game.0, SCISSORS_WL_ENCODING),
            _ => panic!("Failed to compare gestures")
        }
    }
    
    fn compare_gestures(opponent_move: &str, encoding: [&str; 3]) -> u64 {
        let opponent_move_i: i8 = encoding.iter().position(|i| i == &opponent_move).unwrap() as i8;
        let outcome: i8 = 1 - opponent_move_i;
        match outcome {
            -1 => 0u64,
            0 => 3u64,
            1 => 6u64,
            _ => panic!("Error in comparing gestures")
        }
    }

    fn p2_translate_gesture(game: &(&str, &str)) -> (String, String){
        let opponent_move = game.0;
        let wl_key: [&str; 3] = match opponent_move{
            "A" => ENCODED_WL[0],
            "B" => ENCODED_WL[1],
            "C" => ENCODED_WL[2],
            _ => panic!("Error finding wl key")
        };

        let my_move: &str = match game.1 {
            "X" => wl_key[2],
            "Y" => wl_key[1],
            "Z" => wl_key[0],
            _ => panic!("Error matching win condition.")
        };

        return (opponent_move.to_string(), my_move.to_string());

    }

    //creates a vector of tuples repersenting the game gestures from the input
    let game_plan: Vec<(&str, &str)> = include_str!("input.txt")
        .split("\n")
        .map(|game| game.split_once(" ").unwrap())
        .collect();

    // updates the second tuple value to be A,B,C instead of X,Y,Z

    let p1_translated_game_list: Vec<(&str, &str)> = game_plan.iter()
        .map(|game|(game.0, p1_translate_gesture(game.1)))
        .collect();
    let p1_gesture_total = calc_gesture_score(&p1_translated_game_list);
    let p1_wld_total = calc_wld_total(&p1_translated_game_list);
    let p1_total = p1_gesture_total + p1_wld_total;
    let p2_translated_game_list: Vec<(String, String)> = game_plan.iter()
        .map(|game| p2_translate_gesture(game))
        .collect();
    let p2_gesture_total = calc_gesture_score(&p2_translated_game_list);
    let p2_wld_total = calc_wld_total(&p2_translated_game_list);
    let p2_total = p2_gesture_total + p2_wld_total;



    println!("P1 Gesture Total: {:?}", p1_gesture_total);
    println!("P1 Game Condition Total: {:?}", p1_wld_total);
    println!("P1 Total: {:?}", p1_total)
    println!("P1 Gesture Total: {:?}", p2_gesture_total);
    println!("P1 Game Condition Total: {:?}", p2_wld_total);
    println!("P1 Total: {:?}", p2_total)
}


fn p1_translate_gesture(gesture: &str) -> &str {
    match gesture{
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => panic!("Invalid rochambeau gesture found."),
    }
}





fn calc_gesture_score(game_list: &Vec<(&str, &str)>) -> u64 {
    let total: u64 = game_list.iter()
    .map(|game|(
        match game.1{
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("Invalid gesture found!")
        }
    ))
    .collect::<Vec<u64>>()
    .iter()
    .sum();

    return total;
}


