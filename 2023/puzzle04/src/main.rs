use std::collections::HashSet;

fn main()
{
    let cards = include_str!("../input.txt").lines().map(parse_card).collect::<Vec<usize>>();

    println!("{}", cards.iter().map(|&c| 2_usize.pow(c as u32) / 2).sum::<usize>());
    println!("{}", cards.len() + (0 .. cards.len()).map(|i| cards_won(i, &cards)).sum::<usize>())
}

fn parse_card(s : &str) -> usize
{
    let (a, b)  = s.split_once(": ").unwrap().1.split_once(" | ").unwrap();
    let winning = a.split_whitespace().collect::<HashSet<&str>>();

    b.split_whitespace().filter(|k| winning.contains(k)).count()
}

fn cards_won(i : usize, cards : &[usize]) -> usize
{
    (i+1 ..).take(cards[i]).map(|j| 1 + cards_won(j, cards)).sum::<usize>()
}
