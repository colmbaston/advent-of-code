fn main()
{
    let mut input = include_str!("../input.txt").lines()
                                                .map(|s| s.bytes().fold(0, |a, x| 2 * a + (x == b'B' || x == b'R') as u32))
                                                .collect::<Vec<_>>();

    input.sort();
    println!("{}", input[input.len() - 1]);

    let mut prev = input[0];
    for s in input.into_iter().skip(1)
    {
        if s - prev == 2
        {
            println!("{}", prev + 1);
            break
        }
        prev = s;
    }
}
