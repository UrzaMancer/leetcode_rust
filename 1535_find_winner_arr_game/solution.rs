// 11ms 33.33%
// 3.44 MB 66.67%

struct GameState {
    champion: Option<i32>,
    wins: i32,
}

// Introducing this game struct helps to improve readability
// and streamline the design process, preventing certain types of bugs.
// My first iteration of the struct had two raw i32 values, and I decided
// to use an Option after catching a bug where new() needed to be initialized
// with a champion or use magic number to set the wins state. Had I started with
// Option from the beginning, I may not have introduced that bug at all.
//
// My second iteration included a rechallengers Vec() (first a VecDeque, as
// the action of pushing the nums to the back of the array acts as a queue, but
// I switched it to a Vec because I was using iterators, so no pop_front was needed,
// and the assumption I would
// only need to cycle through one additional time, at first.)
//
// I quickly realized after looking at some test cases that the number of wins
// doesn't really matter after you've gone through the whole array, as the numbers
// are distinct, and so the winner will always be the max number.
//
// This also hints that there may be efficiencies to be extracted in jumping
// ahead to find the max in many cases, but I haven't worked out whether that
// pays off yet.
//
// Also note that implementing the rechallengers queue requires additional
// duplication of data because we do not want to borrow the struct to access 
// the queue and also mutate the struct with each .next_round(). Thankfully,
// an easily avoidable mess.



impl GameState {
    pub fn new() -> GameState {
        GameState {
            champion: None,
            wins: 0,
        }
    }

    pub fn next_round(&mut self, challenger: i32) -> i32 {
        if let Some(champ) = self.champion {
            if champ > challenger {
                self.wins += 1;
            } else {
                self.wins = 1;
                self.champion = Some(challenger);
            }
        }
        else {
            self.champion = Some(challenger);
            self.wins = 0;
        }
        self.wins
    }
}

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut game: GameState = GameState::new();
        if let Some(winner) = 
            arr.iter().find_map(|&contender| {
                if game.next_round(contender) == k {
                    game.champion
                } else { None }
            }) { winner }
        else { game.champion.unwrap() }
    }
}
