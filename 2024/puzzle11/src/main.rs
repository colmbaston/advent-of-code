use std::collections::HashMap;

fn main()
{
    let stones = include_str!("../input.txt").split_whitespace()
                                             .map(|t| t.parse::<u64>().unwrap())
                                             .collect::<Vec<u64>>();

    let mut cache = HashMap::new();
    println!("{}", stones.iter().map(|&s| blink(s, 25, &mut cache)).sum::<u64>());
    println!("{}", stones.iter().map(|&s| blink(s, 75, &mut cache)).sum::<u64>());
}

fn blink(stone : u64, iter : u32, cache : &mut HashMap<(u64, u32), u64>) -> u64
{
    if iter == 0                               { return  1 }
    if let Some(k) = cache.get(&(stone, iter)) { return *k }

    let result = if stone == 0
    {
        blink(1, iter - 1, cache)
    }
    else
    {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0
        {
            let div = 10_u64.pow(digits / 2);
            blink(stone / div, iter - 1, cache) + blink(stone % div, iter - 1, cache)
        }
        else
        {
            blink(stone * 2024, iter - 1, cache)
        }
    };

    cache.insert((stone, iter), result);
    result
}
