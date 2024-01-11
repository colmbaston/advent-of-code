fn main()
{
    let input = include_str!("../input.txt").trim_end().parse().unwrap();
    println!("{}", josephus_one(input));
    println!("{}", josephus_two(input));
}

fn josephus_one(n : u32) -> u32
{
    (n - (1 << n.ilog(2))) << 1 | 1
}

fn josephus_two(n : u32) -> u32
{
    let pow = 3u32.pow(n.ilog(3));
    if n == pow
    {
        pow
    }
    else if n - pow <= pow
    {
        n - pow
    }
    else
    {
        2 * n - 3 * pow
    }
}
