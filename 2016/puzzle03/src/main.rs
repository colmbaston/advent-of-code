#![feature(array_chunks)]

fn main()
{
    let input = include_str!("../input.txt").lines()
                                            .map(|l| l.split_whitespace()
                                                      .map(|k| k.parse::<u32>().unwrap())
                                                      .collect())
                                            .collect::<Vec<Vec<u32>>>();

    let transposed = aoc::transpose::transpose(input.iter()
                                                    .map(|a| a.as_slice()))
                                    .collect::<Vec<Vec<u32>>>();

    for matrix in [input, transposed]
    {
        println!("{}", matrix.iter()
                             .flat_map(|a| a.array_chunks())
                             .copied()
                             .filter(|&a| valid(a))
                             .count());
    }
}

fn valid([a, b, c] : [u32 ; 3]) -> bool
{
    let max = a.max(b).max(c);
    let sum = a + b + c;

    max < (sum+1) / 2
}
