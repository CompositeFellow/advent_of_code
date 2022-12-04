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

fn main() {
    const ENCODED_WL : [[&str; 3];3] = [["C","A","B"],["A","B","C"], ["B","C","A"]];

    let given_strategy: Vec<(&str, &str)> = include_str!("input.txt")
        .split("\n")
        .map(|game| game.split_once(" ").unwrap())
        .collect();
    let p1_strat: Vec<(&str, &str)> = given_strategy.iter()
        .map(|game| (game.0, p1_translate_strat(game.1)))
        .collect();
    let p2_strat: Vec<(&str, &str)> = given_strategy.iter()
        .map(|game| (game.0, p2_translate_strat(game)))
        .collect();
    let p1_score: u64 = calc_score(p1_strat);
    let p2_score: u64 = calc_score(p2_strat);

    fn p1_translate_strat(code: &str) -> &str{
        match code{
            "X" => "A",
            "Y" => "B",
            "Z" => "C",
            _ => panic!("P1 failure to translate strategy")
        }
    }


    fn p2_translate_strat<'a>(game: &(&str, &str)) ->&'a str{
        let wl_key = ["A", "B", "C"].iter().enumerate()
            .find(|(_, &key)| key == game.0)
            .map(|(i, _)| ENCODED_WL[i])
            .unwrap_or_else(|| panic!("Failed to find encoding"));
    
        let new_move = ["X", "Y", "Z"].iter().enumerate()
            .find(|(_, &key)| key == game.1)
            .map(|(i, _)| &wl_key[i])
            .unwrap_or_else(|| panic!("P2 Failed to find strategy"));
    
        return new_move;
    }

    fn calc_score(strat: Vec<(&str, &str)>) -> u64 {
        let mut score:u64 = 0;
        for game in strat{
            match game.1{
                "A" => score +=1,
                "B" => score +=2,
                "C" => score +=3,
                _ => panic!("Failed to tally gesture score.")
            }
            
            if game.0 == game.1{
                score += 3
            } else if game.0 == "A" && game.1 == "B" || game.0 == "B" && game.1 == "C" || game.0 == "C" && game.1 == "A"{
                score += 6
            }
        }
        return score;
    }


    println!("P1 Score {:?}", &p1_score);
    print!("P2 Score {:?}", &p2_score);
}

