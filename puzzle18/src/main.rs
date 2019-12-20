use std::collections::{ HashMap, BTreeSet, VecDeque };

fn main()
{
    let input = include_str!("../input.txt");

    let mut entrance = (0, 0);
    let mut keys     = HashMap::new();
    let mut vault    = input.bytes().fold(((0, 0), HashMap::new()), |((x, y), mut m), b|
    {
        if b == b'\n'
        {
            return ((0, y+1), m)
        }
        else if b == b'@'
        {
            entrance = (x, y);
        }
        else if b.is_ascii_lowercase()
        {
            keys.insert(b, (x, y));
        }
        if b != b'#'
        {
            m.insert((x, y), b);
        }

        ((x+1, y), m)
    })
    .1;

    let mut cache = HashMap::new();
    let matrix = adjacency_matrix(vec![entrance], &vault, &keys);
    println!("{}", collect_keys(vec![b'0', b'1', b'2', b'3'], keys.keys().copied().collect(), &matrix, &mut cache));

    cache.clear();
    let (x, y) = entrance;
    vec![(x-1, y), (x, y), (x+1, y), (x, y-1), (x, y+1)].into_iter().for_each(|c| { vault.remove(&c); });
    let matrix = adjacency_matrix(vec![(x-1, y-1), (x-1, y+1), (x+1, y-1), (x+1, y+1)], &vault, &keys);
    println!("{}", collect_keys((b'0'..b'4').collect(), keys.keys().copied().collect(), &matrix, &mut cache));
}

fn adjacency_matrix(entrances : Vec<(i64, i64)>, vault : &HashMap<(i64, i64), u8>, keys : &HashMap<u8, (i64, i64)>) -> HashMap<(u8, u8), (u64, Vec<u8>)>
{
    let mut matrix = HashMap::new();

    let l = entrances.len();
    for (i, (k1, v1)) in (b'0'..).zip(entrances.into_iter()).chain(keys.iter().map(|(k, v)| (*k, *v))).enumerate()
    {
        for (k2, v2) in keys.iter().map(|(k, v)| (*k, *v)).skip((1+i).saturating_sub(l))
        {
            if let (steps, Some((_, path))) = bfs(v1, |&(x, y)| vec![(x+1, y), (x-1, y), (x, y+1), (x, y-1)].into_iter().filter_map(|c| vault.get(&c).map(|_| c)), |c| *c == v2, |c| vault.get(c).filter(|b| b.is_ascii_uppercase()).copied())
            {
                matrix.insert((k1, k2), (steps, path));
            }
        }
    }

    matrix
}

fn bfs<S, I, P>(start : S, adjacent : impl Fn(&S) -> I, complete : impl Fn(&S) -> bool, append_path : impl Fn(&S) -> Option<P>) -> (u64, Option<(S, Vec<P>)>)
where S : Ord, I : Iterator<Item = S>, P : Clone
{
    let mut visited = BTreeSet::new();
    let mut queue   = VecDeque::new();
    queue.push_back((0, start, Vec::new()));

    loop
    {
        if let Some((steps, state, path)) = queue.pop_front()
        {
            if complete(&state)
            {
                return (steps, Some((state, path)))
            }

            queue.extend(adjacent(&state).filter_map(|state|
            {
                if visited.contains(&state)
                {
                    None
                }
                else
                {
                    let mut path = path.clone();
                    if let Some(x) = append_path(&state) { path.push(x) }
                    Some((steps+1, state, path))
                }
            }));

            if queue.is_empty()
            {
                return (steps, None)
            }

            visited.insert(state);
        }
    }
}

fn collect_keys(current : Vec<u8>, keys : BTreeSet<u8>, matrix : &HashMap<(u8, u8), (u64, Vec<u8>)>, cache : &mut HashMap<(Vec<u8>, BTreeSet<u8>), u64>) -> u64
{
    if let Some(&x) = cache.get(&(current.clone(), keys.clone()))
    {
        return x
    }

    if keys.is_empty() { return 0 }

    let result = keys.iter().filter_map(|&k|
    {
        let mut current = current.clone();
        let (d, p) =
        {
            let mut adj = None;
            for x in current.iter_mut()
            {
                if let Some((d, p)) = matrix.get(&(*x, k)).or(matrix.get(&(k, *x)))
                {
                    *x  = k;
                    adj = Some((*d, p.clone()));
                    break
                }
            }
            adj?
        };

        if p.iter().any(|x| keys.contains(&x.to_ascii_lowercase()))
        {
            None
        }
        else
        {
            let mut keys = keys.clone(); keys.remove(&k);
            Some(d + collect_keys(current, keys, &matrix, cache))
        }
    })
    .fold(std::u64::MAX, std::cmp::min);

    cache.insert((current, keys), result);
    result
}
