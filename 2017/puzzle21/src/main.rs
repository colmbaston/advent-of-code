use std::collections::{ HashMap, HashSet };

fn main()
{
    let mut rules = HashMap::new();
    for (mut a, b) in include_str!("../input.txt").lines().map(parse_rule)
    {
        for rev in [true, false].into_iter().cycle().take(8)
        {
            if rev { a.reverse() } else { a = aoc::transpose::transpose(a.iter().map(Vec::as_slice)).collect() }
            rules.insert(a.clone(), b.clone());
        }
    }

    let mut size   = 3;
    let mut grid   = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)].into_iter().collect::<HashSet<Pos>>();
    let mut buffer = HashSet::new();
    for k in 1 ..= 18
    {
        let subsize = if size % 2 == 0 { 2 } else { 3 };
        for gy in 0 .. size / subsize
        {
            for gx in 0 .. size / subsize
            {
                let pattern = (0 .. subsize).map(|y| (0 .. subsize).map(|x| grid.contains(&(x + gx * subsize, y + gy * subsize)))
                                                                   .collect::<Vec<bool>>())
                                            .collect::<Vec<Vec<bool>>>();

                for (row, y) in rules[&pattern].iter().zip(gy * (subsize+1) ..)
                {
                    for x in row.iter().zip(gx * (subsize+1) ..).filter_map(|(b, x)| b.then_some(x))
                    {
                        buffer.insert((x, y));
                    }
                }
            }
        }

        size += size / subsize;
        std::mem::swap(&mut grid, &mut buffer);
        buffer.clear();

        if k == 5 || k == 18 { println!("{}", grid.len()) }
    }
}

type Pos    = (usize, usize);
type Square = Vec<Vec<bool>>;

fn parse_square(s : &str) -> Square
{
    s.split('/').map(|r| r.bytes().map(|b| b == b'#').collect()).collect()
}

fn parse_rule(s : &str) -> (Square, Square)
{
    let (a, b) = s.split_once(" => ").unwrap();
    (parse_square(a), parse_square(b))
}
