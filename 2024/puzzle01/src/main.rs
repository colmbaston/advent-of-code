#![feature(iter_array_chunks)]

fn main()
{
    let (mut list_one, mut list_two) = parse_lists(include_str!("../input.txt"));
    list_one.sort_unstable();
    list_two.sort_unstable();
    println!("{}", list_one.iter().zip(list_two.iter()).map(|(a, b)| (a-b).abs()).sum::<i32>());
    println!("{}", list_one.iter().map(|&a| a * list_two.iter().filter(|&&b| a == b).count() as i32).sum::<i32>());
}

fn parse_lists(s : &str) -> (Vec<i32>, Vec<i32>)
{
    let mut list_one = Vec::new();
    let mut list_two = Vec::new();

    for [a, b] in s.split_whitespace().array_chunks()
    {
        list_one.push(a.parse().unwrap());
        list_two.push(b.parse().unwrap());
    }

    (list_one, list_two)
}
