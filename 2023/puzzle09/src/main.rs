fn main()
{
    let mut seqs = include_str!("../input.txt").lines()
                                               .map(|l| l.split_whitespace()
                                                         .map(|k| k.parse::<i32>().unwrap())
                                                         .collect::<Vec<i32>>())
                                               .collect::<Vec<Vec<i32>>>();

    println!("{}", seqs.iter().map(|s| next_value(s)).sum::<i32>());
    seqs.iter_mut().for_each(|s| s.reverse());
    println!("{}", seqs.iter().map(|s| next_value(s)).sum::<i32>());
}

fn next_value(seq : &[i32]) -> i32
{
    if seq.iter().all(|&k| k == 0) { return 0 }

    seq.last().unwrap() + next_value(&seq.array_windows()
                                         .map(|[a, b]| b - a)
                                         .collect::<Vec<i32>>())
}
