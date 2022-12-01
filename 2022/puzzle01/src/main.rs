fn main()
{
    let mut elves = include_str!("../input.txt").split("\n\n")
                                                .map(|e| e.lines().map(|l| l.parse::<u32>().unwrap()).sum())
                                                .collect::<Vec<u32>>();

    elves.sort_unstable();
    println!("{}", elves.last().unwrap());
    println!("{}", elves.iter().rev().take(3).sum::<u32>());
}
