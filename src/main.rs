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
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let w = logus::Wordle::new();
    for ans in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();
        if let Some(score) = w.play(ans, guesser) {
            println!("guessed '{}' in {}", ans, score);
        } else {
            eprintln!("failed to guess");
        }
    }
}
