fn main()
{
    let input = include_str!("../input.txt").lines()
                                            .flat_map(|l| l.split_once(':').unwrap().1
                                                           .split_whitespace()
                                                           .map(|k| k.parse::<u64>().unwrap()))
                                            .collect::<Vec<u64>>();

    let (times, records) = input.split_at(input.len() / 2);
    println!("{}", times.iter().zip(records.iter())
                        .map(|(&time, &record)| outcomes(time, record))
                        .product::<usize>());

    println!("{}", outcomes(concat(times), concat(records)));
}

fn outcomes(time : u64, record : u64) -> usize
{
    (0 ..= time).map(move |hold| hold * (time - hold))
                .skip_while(|&dist| dist <= record)
                .take_while(|&dist| dist >  record)
                .count()
}

fn concat(ns : &[u64]) -> u64
{
    ns.iter().fold(0, |a, n| 10_u64.pow(1 + n.ilog10()) * a + n)
}
