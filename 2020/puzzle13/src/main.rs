fn main()
{
    let (k, bs) = parse_schedule(include_str!("../input.txt"));

    let (b, m) = bs.iter().filter_map(|b| b.map(|t| (t, t - k % t))).min_by(|x, y| x.1.cmp(&y.1)).unwrap();
    println!("{}", b * m);

    let mut congs = bs.iter().zip(0..).filter_map(|(b, t)| b.map(|n| (n, (n - t % n) % n))).collect::<Vec<_>>();
    congs.sort_unstable();
    let (mut acc_n, mut acc_t) = congs.pop().unwrap();
    for (n, t) in congs.into_iter().rev()
    {
        acc_t  = (0 ..).map(|k| k * acc_n + acc_t).find(|k| k % n == t).unwrap();
        acc_n *= n;
    }
    println!("{}", acc_t);
}

fn parse_schedule(s : &str) -> (u64, Vec<Option<u64>>)
{
    let mut i = s.lines();
    (i.next().unwrap().parse().unwrap(),
     i.next().unwrap().split(',').map(|x| x.parse().ok()).collect())
}
