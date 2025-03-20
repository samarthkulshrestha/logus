const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = logus::Wordle::new();
    for ans in GAMES.split_whitespace() {
        let guesser = logus::algorithms::Naive::new();
        if let Some(score) = w.play(ans, guesser) {
            println!("guessed '{}' in {}", ans, score);
        } else {
            eprintln!("failed to guess");
        }
    }
}
