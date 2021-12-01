fn main()
{
    let input = include_str!("../input.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut count = 0;
    let mut prev  = input[0];
    for &a in input.iter().skip(1)
    {
        if a > prev { count += 1 }
        prev = a;
    }
    println!("{}", count);

    count = 0;
    prev  = input.iter().take(3).sum();
    for (a, b) in input.iter().zip(input.iter().skip(3))
    {
        let c = prev - a + b;
        if c > prev { count += 1 }
        prev = c;
    }
    println!("{}", count);
}
