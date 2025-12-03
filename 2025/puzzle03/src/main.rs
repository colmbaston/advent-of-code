fn main()
{
    let mut sum_one = 0;
    let mut sum_two = 0;
    for bank in include_str!("../input.txt").lines().map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
    {
        sum_one += best_joltage(&bank,  2);
        sum_two += best_joltage(&bank, 12);
    }
    println!("{sum_one}");
    println!("{sum_two}");
}

fn best_joltage(bank : &[u8], size : usize) -> u64
{
    let mut best = bank[.. size].to_vec();
    for &joltage in &bank[size ..]
    {
        best.push(joltage);
        for i in 0 .. size
        {
            if best[i] < best[i+1]
            {
                best.remove(i);
                break
            }
        }
        if best.len() > size { best.pop(); }
    }
    best.into_iter().fold(0, |a, j| a*10 + j as u64)
}
