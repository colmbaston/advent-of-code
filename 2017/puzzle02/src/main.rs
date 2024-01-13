fn main()
{
    let rows = include_str!("../input.txt").lines()
                                           .map(|l| l.split_whitespace()
                                                     .map(|k| k.parse::<u32>().unwrap())
                                                     .collect::<Vec<u32>>())
                                           .collect::<Vec<Vec<u32>>>();

    println!("{}", rows.iter().map(|r| checksum(r)).sum::<u32>());
    println!("{}", rows.iter().map(|r| divide(r)).sum::<u32>());
}

fn checksum(row : &[u32]) -> u32
{
    let (min, max) = row.iter()
                        .map(|k| (k, k))
                        .reduce(|(a, b), (c, d)| (a.min(c), b.max(d)))
                        .unwrap();
    max - min
}

fn divide(row : &[u32]) -> u32
{
    let (a, b) = row.iter().enumerate()
                    .flat_map(|(i, a)| row.iter().skip(i+1).map(move |b| (a.max(b), a.min(b))))
                    .find(|&(a, b)| a % b == 0)
                    .unwrap();

    a / b
}
