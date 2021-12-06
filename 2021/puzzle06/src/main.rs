fn main()
{
    let mut fish = [0 ; 9];
    for i in include_str!("../input.txt").trim_end().split(',').map(|s| s.parse::<usize>().unwrap())
    {
        fish[i] += 1;
    }

    for g in 0 .. 256
    {
        if g == 80 { println!("{}", fish.iter().sum::<u64>()) }
        fish.rotate_left(1);
        fish[6] += fish[8];
    }
    println!("{}", fish.iter().sum::<u64>());
}
