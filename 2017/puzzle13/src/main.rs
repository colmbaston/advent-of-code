fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_line).collect::<Vec<(u32, u32)>>();
    println!("{}", input.iter().map(|(depth, range)| if depth % (2 * (range-1)) == 0 { depth * range } else { 0 }).sum::<u32>());
    println!("{}", (0 ..).find(|delay| input.iter().all(|(depth, range)| (depth + delay) % (2 * (range - 1)) != 0)).unwrap());
}

fn parse_line(s : &str) -> (u32, u32)
{
    let (a, b) = s.split_once(": ").unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}
