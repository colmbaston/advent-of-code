use std::collections::{ HashSet, HashMap };
use aoc::{ direction::Direction, permutations::Permutations };

fn main()
{
    let (grid, nums) = parse_grid(include_str!("../input.txt"));

    let mut dists = HashMap::<(u8, u8), u32>::new();
    for (i, (&k, &p1)) in nums.iter().enumerate()
    {
        for (&l, &p2) in nums.iter().skip(i+1)
        {
            let dist = aoc::pathfinding::bfs(std::iter::once(p1),
                                             |&pos| pos == p2,
                                             |&pos| Direction::ELEMS.into_iter()
                                                                    .map(move |dir| dir.step(pos))
                                                                    .filter(|pos| grid.contains(pos))).unwrap();

            dists.insert((k, l), dist);
            dists.insert((l, k), dist);
        }
    }

    let (min_one, min_two) = Permutations::from_unsorted(nums.keys().copied().filter(|&k| k != 0)).map(|perm|
    {
        let dist  = perm.array_windows().map(|&[k, l]| dists[&(k, l)]).sum::<u32>();
        let start = dists[&(0, *perm.first().unwrap())];
        let end   = dists[&(0, *perm.last().unwrap())];
        (dist + start, dist + start + end)
    })
    .reduce(|(a, b), (c, d)| (a.min(c), b.min(d)))
    .unwrap();

    println!("{min_one}");
    println!("{min_two}");
}

type Pos = (i32, i32);

fn parse_grid(s : &str) -> (HashSet<Pos>, HashMap<u8, Pos>)
{
    let mut grid = HashSet::new();
    let mut nums = HashMap::new();

    for (l, y) in s.lines().zip(0 ..)
    {
        for (b, x) in l.bytes().zip(0 ..)
        {
            if b.is_ascii_digit()
            {
                grid.insert((x, y));
                nums.insert(b - b'0', (x, y));
            }
            else if b == b'.'
            {
                grid.insert((x, y));
            }
        }
    }

    (grid, nums)
}
