use std::{borrow::Cow, collections::HashSet};

pub mod algorithms;

const DICT: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dict: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dict: HashSet::from_iter(DICT.lines().map(|line| {
                line.split_once(' ')
                    .expect("every line is word + space + freq")
                    .0
            })),
        }
    }

    pub fn play<G: Guesser>(&self, ans: &'static str, mut guesser: G) -> Option<usize>{
        let mut hist = Vec::new();
        for i in 1..=32 {
            let guess = guesser.guess(&hist);
            if guess == ans {
                return Some(i);
            }
            assert!(self.dict.contains(&*guess), "guess '{}' is not in the dict", guess);
            let correctness = Correctness::compute(ans, &guess);
            hist.push(Guess { word: Cow::Owned(guess), mask: correctness });
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Correctness {
    Correct,            // green
    Misplaced,          // yellow
    Incorrect,          // grey
}

impl Correctness {
    fn compute(ans: &str, guess: &str) -> [Self; 5] {
        assert_eq!(ans.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Incorrect; 5];
        let mut used = [false; 5];

        // mark as correct
        for (i, (a, g)) in ans.bytes().zip(guess.bytes()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
                used[i] = true;
            }
        }

        for (i, g) in guess.bytes().enumerate() {
            if c[i] == Correctness::Correct {
                // already marked correct
                continue;
            }
            if ans.bytes().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }

        c
    }


    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Incorrect],
            [Self::Correct, Self::Misplaced, Self::Incorrect],
            [Self::Correct, Self::Misplaced, Self::Incorrect],
            [Self::Correct, Self::Misplaced, Self::Incorrect],
            [Self::Correct, Self::Misplaced, Self::Incorrect]
        )
            .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

pub struct Guess<'a> {
    pub word: Cow<'a, str>,
    pub mask: [Correctness; 5],
}

impl Guess<'_> {
    pub fn matches(&self, word: &str) -> bool {
        return Correctness::compute(word, &self.word) == self.mask;
    }
}

pub trait Guesser {
    fn guess(&mut self, hist: &[Guess]) -> String;
}

impl Guesser for fn(hist: &[Guess]) -> String {
    fn guess(&mut self, hist: &[Guess]) -> String {
        (*self)(hist)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$hist:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $hist: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
macro_rules! mask {
    (C) => {$crate::Correctness::Correct};
    (M) => {$crate::Correctness::Misplaced};
    (I) => {$crate::Correctness::Incorrect};
    ($($c:tt)+) => {[
        $(mask!($c)),+
    ]}
}

#[cfg(test)]
mod tests {

    mod guess_matcher {
        use crate::Guess;
        use std::borrow::Cow;

        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {
                word: Cow::Borrowed($prev),
                mask: mask![$($mask )+]
                }
                .matches($next));
                assert_eq!($crate::Correctness::compute($next, $prev), mask![$($mask )+]);
            };
            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {
                word: Cow::Borrowed($prev),
                mask: mask![$($mask )+]
                }
                .matches($next));
                assert_ne!($crate::Correctness::compute($next, $prev), mask![$($mask )+]);
            }
        }

        #[test]
        fn guess_matcher_tests() {
            check!("abcde" + [C C C C C] allows "abcde");
            check!("abcdf" + [C C C C C] disallows "abcde");
            check!("abcde" + [I I I I I] allows "fghij");
            check!("abcde" + [M M M M M] allows "eabcd");
            check!("abcde" + [M M M M M] allows "eabcd");
            check!("baaaa" + [I C M I I] allows "aaccc");
            check!("baaaa" + [I C M I I] disallows "caacc");
            check!("aaabb" + [C M I I I] disallows "accaa");
            check!("abcde" + [I I I I I] disallows "bcdea");
            check!("tares" + [I M M I I] disallows "brink");
        }
    }

    mod game {
        use crate::{Guess, Wordle};

        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_hist| {
                "right".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(1));
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|hist| {
                if hist.len() == 1 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|hist| {
                if hist.len() == 2 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(3));
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|hist| {
                if hist.len() == 3 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(4));
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|hist| {
                if hist.len() == 4 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(5));
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|hist| {
                if hist.len() == 5 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(6));
        }

        #[test]
        fn wrong() {
            let w = Wordle::new();
            let guesser = guesser!(|_hist| { "wrong".to_string() });
            assert_eq!(w.play("right", guesser), None);
        }
    }

    mod compute {
        use crate::Correctness;

        #[test]
        fn all_correct() {
            assert_eq!(
                Correctness::compute("abcde", "abcde"),
                mask![C C C C C]
            );
        }

        #[test]
        fn all_incorrect() {
            assert_eq!(
                Correctness::compute("abcde", "fghij"),
                mask![I I I I I]
            );
        }

        #[test]
        fn all_misplaced() {
            assert_eq!(
                Correctness::compute("abcde", "eabcd"),
                mask![M M M M M]
            );
        }

        #[test]
        fn repeat_correct() {
            assert_eq!(
                Correctness::compute("aabbb", "aaccc"),
                mask![C C I I I]
            );
        }

        #[test]
        fn repeat_misplaced() {
            assert_eq!(
                Correctness::compute("aabbb", "ccaac"),
                mask![I I M M I]
            );
        }

        #[test]
        fn repeat_some_correct() {
            assert_eq!(
                Correctness::compute("aabbb", "caacc"),
                mask![I C M I I]
            );
        }

        #[test]
        fn random() {
            assert_eq!(
                Correctness::compute("azzaz", "aaabb"),
                mask![C M I I I]
            );
        }
    }
}
