#![feature(iter_array_chunks)]

fn main()
{
    let (mut left, mut right) : (Vec<u32>, Vec<u32>) = include_str!("../input.txt").split_whitespace()
                                                                                   .map(|t| t.parse::<u32>().unwrap())
                                                                                   .array_chunks()
                                                                                   .map(|[a, b]| (a, b))
                                                                                   .unzip();

    left.sort_unstable();
    right.sort_unstable();
    println!("{}", left.iter()
                       .zip(right.iter())
                       .map(|(a, b)| a.abs_diff(*b))
                       .sum::<u32>());

    let mut i = 0;
    println!("{}", left.iter().fold(0, |a, &l|
    {
        let c = right[i..].iter()
                          .skip_while(|&&r| l >  r)
                          .take_while(|&&r| l == r)
                          .count();

        i += c;
        a + l * c as u32
    }))
}
