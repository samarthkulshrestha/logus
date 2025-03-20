# logus: entropy-based wordle-solving algorithm in rust

This project implements and benchmarks multiple algorithms to solve the game
Wordle using principles of information theory, particularly entropy. Inspired
by the viral puzzle game, this project explores optimal strategies to reduce
uncertainty with each guess, aiming to solve the game in as few attempts as
possible.

## overview of wordle
Wordle is a word-guessing game where:
1. Players have six attempts to guess a five-letter mystery word.
2. Feedback is given for each guess:
   - **Green**: Correct letter placed in the correct position.
   - **Yellow**: Correct letter but placed in the wrong position.
   - **Gray**: Letter is not in the mystery word.

## approach

### information theory in wordle
The algorithms use **entropy** to evaluate guesses. Entropy measures the
"expected information gain" from a guess, aiming to maximize the reduction of
possible solutions. Here's how it works:
1. **Entropy Calculation**:
   - For each guess, calculate the possible feedback patterns (e.g., Green-Yellow-Gray combinations).
   - Use the probability of each pattern to compute its entropy using the formula:
     ```math
     H = -\sum P(x) \log_2 P(x)
     ```
   - Higher entropy means greater potential to reduce the search space.
2. **Word Selection**:
   - Start with a uniform probability distribution of all valid five-letter words (~13,000).
   - Rank guesses based on their expected entropy to prioritize guesses that maximize information gain.

### refinements
- Incorporated **word frequency** data to prioritize more common English words as potential answers.
- Used a **sigmoid function** to smooth the weighting of word frequencies, avoiding over-reliance on uncommon words.
- Implemented a probabilistic model to refine the decision-making process in later guesses, balancing entropy maximization and solution likelihood.

### algorithm versions
1. **Vecrem**:
   - Basic entropy-based approach, with minor optimisations on the naive
   algorithm.
   - No consideration of word frequencies.
   - Average performance: ~4.12 guesses per game.
2. **Weight**:
   - Incorporates word frequency data.
   - Probabilistic refinement for endgame decisions.
   - Average performance: ~3.6 guesses per game.
3. **Sigmoid (Advanced)**:
   - Looks two steps ahead to optimize decisions.
   - Utilizes the official Wordle answer list for maximum performance.
   - Average performance: ~3.43 guesses per game.
4. **Popular**
   - Strawman algorithm which chooses the most *popular* word out of the
   remaining words which match.
   - Runs slightly faster than Sigmoid, but also slightly less accurate.

## benchmarks
- **Vecrem**: Average of ~4.12 guesses, achieving par (4 guesses) in most games.
- **Weight**: Improved decision-making with an average of ~3.6 guesses.
- **Sigmoid**: Achieved an average of ~3.43 guesses when leveraging the Wordle answer list.

## key insights
- Entropy is a powerful tool for decision-making under uncertainty, allowing for systematic guess optimization.
- Word frequency data significantly enhances performance, especially in reducing late-game uncertainty.
- Theoretical limits suggest that achieving an average of 3 guesses per game is infeasible due to inherent information constraints.

## usage
1. Clone the repository:
   ```bash
   git clone https://github.com/samarthkulshrestha/logus.git
   cd wordle-solver
   ```
2. Run the solver:
   ```bash
   cargo run --release -i <implementation> -m <max-games>
   ```
3. Usage options:
   ```
   -h, --help
   -i, --implementation <IMPLEMENTATION>
       [possible values: naive, allocs, vecrem, once, precalc, weight, enum,
   cutoff, popular, sigmoid]
   -m, --max
   ```

## credits
This project was inspired by the [3blue1brown](https://www.youtube.com/@3blue1brown) video: [Solving Wordle using information theory](https://youtu.be/v68zYyaEmEA).


### contribute

+ i <3 pull requests and bug reports!
+ don't hesitate to [tell me my code-fu sucks](https://github.com/samarthkulshrestha/logus/issues/new), but please tell me why.

### license

This project is licensed under the MIT License.

Copyright (c) 2025 Samarth Kulshrestha.
