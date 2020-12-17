use std::collections::HashSet;

fn main()
{
    let input = parse_dimension(include_str!("../input.txt"));

    let mut three = input.iter().cloned().map(|mut p| { p.push(0); p }).collect::<HashSet<_>>();
    for _ in 0 .. 6 { three = cycle(&three) }
    println!("{}", three.len());

    let mut four = input.iter().cloned().map(|mut p| { p.push(0); p.push(0); p }).collect::<HashSet<_>>();
    for _ in 0 .. 6 { four = cycle(&four) }
    println!("{}", four.len());
}

type Point     = Vec<i32>;
type Dimension = HashSet<Point>;

fn neighbours(p : &[i32]) -> Vec<Point>
{
    match p.last()
    {
        None     => vec![p.to_vec()],
        Some(&k) =>
        {
            let     prev = neighbours(&p[.. p.len()-1]);
            let mut next = Vec::with_capacity(3 * prev.len());

            for mut p in prev.into_iter()
            {
                p.push(k-1);
                next.push(p.clone());
                *p.last_mut().unwrap() += 1;
                next.push(p.clone());
                *p.last_mut().unwrap() += 1;
                next.push(p);
            }

            next
        }
    }
}

fn cycle(dim : &Dimension) -> Dimension
{
    let mut new = HashSet::new();

    for p in dim.iter().flat_map(|p| neighbours(p).into_iter()).collect::<HashSet<_>>().into_iter()
    {
        let count = neighbours(&p).iter().filter(|&n| p != *n && dim.contains(n)).count();
        if count == 3 || count == 2 && dim.contains(&p) { new.insert(p); }
    }

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
