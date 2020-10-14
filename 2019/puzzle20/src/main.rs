use std::collections::{ VecDeque, HashSet, HashMap };

fn main()
{
    let input = include_str!("../input.txt");

    let mut width  = 0;
    let mut height = 0;
    let mut portal_chars = HashMap::new();
    let maze = input.bytes().fold(((0, 0), HashMap::new()), |((x, y), mut m), b|
    {
        if b == b'\n'
        {
            width  = width.max(x);
            height = y+1;
            return ((0, y+1), m)
        }
        else if b.is_ascii_alphabetic()
        {
            portal_chars.insert((x, y), b);
        }
        if b != b'#' && b != b' '
        {
            m.insert((x, y), b);
        }

        ((x+1, y), m)
    })
    .1;

    let mut start   = (0, 0);
    let mut end     = (0, 0);
    let mut portals = HashMap::new();
    for (&(x, y), &p) in portal_chars.iter()
    {
        let (c, &q) = if let Some(q) = portal_chars.get(&(x+1, y))
        {
            match maze.get(&(x+2, y))
            {
                Some(b'.') => ((x+2, y), q),
                _          => ((x-1, y), q)
            }
        }
        else if let Some(q) = portal_chars.get(&(x, y+1))
        {
            match maze.get(&(x, y+2))
            {
                Some(b'.') => ((x, y+2), q),
                _          => ((x, y-1), q)
            }
        }
        else
        {
            continue
        };

        if p == b'A' && q == b'A'
        {
            start = c;
        }
        else if p == b'Z' && q == b'Z'
        {
            end = c;
        }
        else
        {
            portals.entry((p, q)).or_insert_with(Vec::new).push(c);
        }
    }

    let mut links = HashMap::new();
    for v in portals.values()
    {
        links.insert(v[0], v[1]);
        links.insert(v[1], v[0]);
    }

    let adjacent = |&o : &(i32, i32)|
    {
        let maze  = &maze;
        let links = &links;
        ortho(o).filter_map(move |c|
        {
            match maze.get(&c)
            {
                Some(b'.')                         => Some(c),
                Some(b) if b.is_ascii_alphabetic() => links.get(&o).copied(),
                _                                  => None
            }
        })
    };

    let adjacent_rec = |&((x, y), l) : &((i32, i32), u64)|
    {
        let maze  = &maze;
        let links = &links;
        ortho((x, y)).filter_map(move |c|
        {
            match maze.get(&c)
            {
                Some(b'.')                         => Some((c, l)),
                Some(b) if b.is_ascii_alphabetic() =>
                {
                    if x == 2 || y == 2 || x == width-3 || y == height-3
                    {
                        if l == 0
                        {
                            None
                        }
                        else
                        {
                            links.get(&(x, y)).copied().map(|c| (c, l-1))
                        }
                    }
                    else
                    {
                        links.get(&(x, y)).copied().map(|c| (c, l+1))
                    }
                },
                _ => None
            }
        })
    };

    println!("{}", bfs( start,     adjacent,     |&c|      c == end,         ));
    println!("{}", bfs((start, 0), adjacent_rec, |&(c, l)| c == end && l == 0));
}

fn ortho((x, y) : (i32, i32)) -> impl Iterator<Item = (i32, i32)>
{
    vec![(x+1, y), (x-1, y), (x, y+1), (x, y-1)].into_iter()
}

fn bfs<S, I>(start : S, adjacent : impl Fn(&S) -> I, finish : impl Fn(&S) -> bool) -> u32
where S : Clone + Eq + std::hash::Hash, I : Iterator<Item = S>
{
    let mut visited = HashSet::new();
    let mut queue   = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((state, steps)) = queue.pop_front()
    {
        if !visited.insert(state.clone()) { continue }
        if finish(&state) { return steps }
        queue.extend(adjacent(&state).map(|s| (s, steps+1)));
    }

    panic!("exhausted search")
}
