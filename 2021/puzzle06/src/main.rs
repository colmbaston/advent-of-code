fn main()
{
    let mut fish = [0 ; 9];
    for i in include_str!("../input.txt").trim_end().split(',').map(|s| s.parse::<usize>().unwrap())
    {
        fish[i] += 1;
    }

    for i in [0 .. 80, 80 .. 256].into_iter()
    {
        for p in i { fish[(p + 7) % 9] += fish[p % 9] }
        println!("{}", fish.iter().sum::<u64>());
    }
}
