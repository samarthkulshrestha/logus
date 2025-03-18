const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = logus::Wordle::new();
    for ans in GAMES.split_whitespace() {
        let guesser = logus::algorithms::Naive::new();
        w.play(ans, guesser);
    }
}
