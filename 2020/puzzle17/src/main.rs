use std::collections::HashSet;

fn main()
{
    let input = parse_dimension(include_str!("../input.txt"));

    let mut three = input.iter().cloned().map(|mut p| { p.push(0); p }).collect::<HashSet<_>>();
    for _ in 0 .. 6 { three = cycle(&three) }
    println!("{}", three.len());

    let mut four = input.into_iter().map(|mut p| { p.push(0); p.push(0); p }).collect::<HashSet<_>>();
    for _ in 0 .. 6 { four = cycle(&four) }
    println!("{}", four.len());
}

type Point     = Vec<i32>;
type Dimension = HashSet<Point>;

fn neighbours(p : &[i32]) -> Vec<Point>
{
    match p.split_last()
    {
        None             => vec![p.to_vec()],
        Some((&k, rest)) =>
        {
            let     prev = neighbours(&rest);
            let mut next = Vec::with_capacity(3 * prev.len());
            for mut q in prev.into_iter()
            {
                q.push(k-1);
                next.push(q.clone());
                *q.last_mut().unwrap() += 1;
                next.push(q.clone());
                *q.last_mut().unwrap() += 1;
                next.push(q);
            }
            next
        }
    }
}

fn cycle(dim : &Dimension) -> Dimension
{
    let mut new = dim.iter()
                     .flat_map(|p| neighbours(p).into_iter())
                     .collect::<HashSet<_>>();

    new.retain(|p|
    {
        let count = neighbours(&p).into_iter().filter(|q| p != q && dim.contains(q)).count();
        count == 3 || count == 2 && dim.contains(p)
    });

    new
}

fn parse_dimension(s : &str) -> Dimension
{
    let mut dim = HashSet::new();
    for (l, y) in s.lines().zip(0..)
    {
        for (b, x) in l.bytes().zip(0..)
        {
            if b == b'#' { dim.insert(vec![x, y]); }
        }
    }
    dim
}
