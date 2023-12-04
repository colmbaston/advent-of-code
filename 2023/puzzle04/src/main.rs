use std::collections::HashSet;

fn main()
{
    let cards = include_str!("../input.txt").lines().map(parse_card).collect::<Vec<usize>>();

    println!("{}", cards.iter().map(|&c| 2_usize.pow(c as u32) / 2).sum::<usize>());

    let mut cache = vec![1 ; cards.len()];
    for (i, matches) in cards.into_iter().enumerate().rev()
    {
        cache[i] += (i+1 ..).take(matches).map(|j| cache[j]).sum::<usize>();
    }
    println!("{}", cache.iter().sum::<usize>());
}

fn parse_card(s : &str) -> usize
{
    let (a, b)  = s.split_once(": ").unwrap().1.split_once(" | ").unwrap();
    let winning = a.split_whitespace().collect::<HashSet<&str>>();

    b.split_whitespace().filter(|k| winning.contains(k)).count()
}
