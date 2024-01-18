#![feature(iter_next_chunk)]

fn main()
{
    let [a, b] = include_str!("../input.txt").lines()
                                             .map(|l| l[24 ..].parse::<u64>().unwrap())
                                             .next_chunk().unwrap();

    let mut state_a = a;
    let mut state_b = b;
    let sequence_a  = std::iter::from_fn(|| { state_a *= 16_807; state_a %= 2_147_483_647; Some(state_a) });
    let sequence_b  = std::iter::from_fn(|| { state_b *= 48_271; state_b %= 2_147_483_647; Some(state_b) });
    println!("{}", sequence_a.zip(sequence_b).take(40_000_000).filter(|(a, b)| a & 0xffff == b & 0xffff).count());

    let mut state_a = a;
    let mut state_b = b;
    let sequence_a  = std::iter::from_fn(|| { state_a *= 16_807; state_a %= 2_147_483_647; Some(state_a) }).filter(|a| a % 4 == 0);
    let sequence_b  = std::iter::from_fn(|| { state_b *= 48_271; state_b %= 2_147_483_647; Some(state_b) }).filter(|b| b % 8 == 0);
    println!("{}", sequence_a.zip(sequence_b).take(5_000_000).filter(|(a, b)| a & 0xffff == b & 0xffff).count());
}
