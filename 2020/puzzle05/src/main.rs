fn main()
{
    let mut input = include_str!("../input.txt").lines()
                                                .map(|s| s.bytes().fold(0, |a, x| 2 * a + (x == b'B' || x == b'R') as u32))
                                                .collect::<Vec<_>>();

    input.sort();
    println!("{}", input[input.len() - 1]);
    println!("{}", input.windows(2).find(|w| w[0] + 2 == w[1]).unwrap()[0] + 1);
}
