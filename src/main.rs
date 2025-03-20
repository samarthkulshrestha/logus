use clap::{ArgEnum, Parser};
use logus::Guesser;

const GAMES: &str = include_str!("../answers.txt");

/// simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, arg_enum)]
    implementation: Implementation,

    #[clap(short, long)]
    max: Option<usize>,
}

#[derive(ArgEnum, Debug, Clone, Copy)]
enum Implementation {
    Naive,
    Allocs,
    Vecrem,
    Once,
    Precalc,
    Weight,
    Enum,
    Cutoff,
    Popular,
    Sigmoid,
}

fn main() {
    let args = Args::parse();

    match args.implementation {
        Implementation::Naive => {
            play(logus::algorithms::Naive::new, args.max);
        }
        Implementation::Allocs => {
            play(logus::algorithms::Allocs::new, args.max);
        }
        Implementation::Vecrem => {
            play(logus::algorithms::Vecrem::new, args.max);
        }
        Implementation::Once => {
            play(logus::algorithms::OnceInit::new, args.max);
        }
        Implementation::Precalc => {
            play(logus::algorithms::Precalc::new, args.max);
        }
        Implementation::Weight => {
            play(logus::algorithms::Weight::new, args.max);
        }
        Implementation::Enum => {
            play(logus::algorithms::Enumerate::new, args.max);
        }
        Implementation::Cutoff => {
            play(logus::algorithms::Cutoff::new, args.max);
        }
        Implementation::Popular => {
            play(logus::algorithms::Popular::new, args.max);
        }
        Implementation::Sigmoid => {
            play(logus::algorithms::Sigmoid::new, args.max);
        }
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let w = logus::Wordle::new();
    let mut score = 0;
    let mut games = 0;

    for ans in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();
        if let Some(s) = w.play(ans, guesser) {
            games += 1;
            score += s;
            println!("guessed '{}' in {}", ans, s);
        } else {
            eprintln!("failed to guess");
        }
    }
    println!("average score: {:.4}", score as f64 / games as f64);
}


#[cfg(test)]
mod tests {
    #[test]
    fn first_10_games_with_cutoff() {
        let w = logus::Wordle::new();
        let results: Vec<_> = crate::GAMES
            .split_whitespace()
            .take(10)
            .filter_map(|answer| w.play(answer, logus::algorithms::Cutoff::new()))
            .collect();

        assert_eq!(results, [4, 4, 4, 4, 4, 5, 4, 5, 4, 2]);
    }
}
