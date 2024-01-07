use std::collections::HashMap;

fn main()
{
    let input = aoc::transpose::transpose(include_str!("../input.txt").lines().map(|l| l.as_bytes()));

    let mut min = String::new();
    let mut max = String::new();

    for l in input.into_iter()
    {
        let mut hist = HashMap::new();
        for b in l.into_iter()
        {
            *hist.entry(b).or_insert(0) += 1;
        }

        let mut hist = hist.into_iter().collect::<Vec<(u8, u32)>>();
        hist.sort_unstable_by(|(b1, f1), (b2, f2)| f1.cmp(f2).then(b1.cmp(b2)));

        min.push(hist.first().unwrap().0 as char);
        max.push(hist.last().unwrap().0 as char);
    }

    println!("{max}");
    println!("{min}");
}
