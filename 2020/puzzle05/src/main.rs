fn main()
{
    let mut input = include_str!("../input.txt").lines()
                                                .map(|s| s.bytes().fold(0, |a, x| 2 * a + (x == b'B' || x == b'R') as u32))
                                                .collect::<Vec<_>>();

    input.sort_unstable();
    println!("{}", input.last().unwrap());
    println!("{}", (input[0] ..).zip(input.into_iter()).find(|(s, t)| s != t).unwrap().0);
}
