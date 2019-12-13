fn main()
{
    let input : Vec<u64> = include_str!("../input.txt").lines().map(|x| x.parse().unwrap()).collect();

    println!("{}", input.iter().map(|x| x / 3 - 2).sum::<u64>());
    println!("{}", input.iter().map(|x|  fuel(*x)).sum::<u64>());
}

fn fuel(mut x : u64) -> u64
{
    let mut sum = 0;
    while x > 6
    {
        x = x / 3 - 2;
        sum += x;
    }

    sum
}
