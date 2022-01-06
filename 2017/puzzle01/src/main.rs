fn main()
{
    let input = include_str!("../input.txt").trim_end().bytes().map(|b| (b - b'0') as u32).collect::<Vec<u32>>();
    println!("{}", input.iter().zip(input.iter().cycle().skip(1              )).fold(0, |a, (b, c)| if b == c { a+b } else { a }));
    println!("{}", input.iter().zip(input.iter().cycle().skip(input.len() / 2)).fold(0, |a, (b, c)| if b == c { a+b } else { a }));
}
