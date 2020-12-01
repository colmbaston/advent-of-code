# Advent of Code

My solutions to [Advent of Code](https://adventofcode.com/) puzzles, written in Rust.
Each year's puzzles are organised into a Cargo workspace consisting of one binary crate per puzzle.
Each puzzle's binary:
* Has no out-of-repository dependencies other than the Rust standard library;
* Parses the puzzle input from a file called `input.txt` placed in the crate's root directory;
* With [some](2018/puzzle19/src/main.rs) [notable](2018/puzzle21/src/main.rs) [exceptions](2019/puzzle25/src/main.rs), aims to solve the puzzle for any valid input.

![](https://colmbaston.uk/animation.gif)
