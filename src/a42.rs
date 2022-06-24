// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

struct ScoringSystem{
    current_score : i32,
    powerup_point : i32,
}

impl ScoringSystem {
    fn new() -> Self {
        Self {
            current_score : 1,
            powerup_point : 1,
        }
    }

    fn powerup(&mut self){
        self.powerup_point = self.powerup_point+1;
    }
}

impl Iterator for ScoringSystem {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_score += self.powerup_point;
        Some(self.current_score)
    }
}

fn main() {
    let mut score = ScoringSystem::new();
    println!("{:?}",score.next());
    println!("{:?}",score.next());
    println!("{:?}",score.next());
    println!("{:?}",score.next());
    score.powerup();
    println!("{:?}",score.next());
    println!("{:?}",score.next());
    println!("{:?}",score.next());
    println!("{:?}",score.next());
    score.powerup();
    println!("{:?}",score.next());
    println!("{:?}",score.next());
    println!("{:?}",score.next());
    println!("{:?}",score.next());
}
