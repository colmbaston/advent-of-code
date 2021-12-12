fn main()
{
    let mut fish = [0 ; 9];
    for i in include_str!("../input.txt").trim_end().split(',').map(|s| s.parse::<usize>().unwrap())
    {
        fish[i] += 1;
    }

    let mut p = 0;
    for g in 0 .. 256
    {
        if g == 80 { println!("{}", fish.iter().sum::<u64>()) }

        p = (p + 1) % 9;
        fish[(p + 6) % 9] += fish[(p + 8) % 9];
    }
    println!("{}", fish.iter().sum::<u64>());
}
