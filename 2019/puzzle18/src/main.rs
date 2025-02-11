use std::collections::{ VecDeque, HashSet, HashMap, BTreeSet };

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
    println!("{}", collect_keys(vec![b'0'], keys.keys().copied().collect(), &matrix, &mut cache));

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
            let mut visited = HashSet::new();
            let mut queue   = VecDeque::new();
            queue.push_back((v1, 0, Vec::new()));

            while let Some(((x, y), steps, path)) = queue.pop_front()
            {
                if !visited.insert((x, y)) { continue }
                if (x, y) == v2
                {
                    matrix.insert((k1, k2), (steps, path));
                    break
                }

                for c in vec![(x+1, y), (x-1, y), (x, y+1), (x, y-1)].into_iter().filter_map(|c| vault.get(&c).map(|_| c))
                {
                    let mut p = path.clone();
                    if let Some(&b) = vault.get(&c).filter(|b| b.is_ascii_uppercase())
                    {
                        p.push(b)
                    }

                    queue.push_back((c, steps+1, p))
                }
            }
        }
    }

    matrix
}

fn collect_keys(current : Vec<u8>, keys : BTreeSet<u8>, matrix : &HashMap<(u8, u8), (u64, Vec<u8>)>, cache : &mut HashMap<Vec<u8>, HashMap<BTreeSet<u8>, u64>>) -> u64
{
    if keys.is_empty() { return 0 }

    if let Some(&x) = cache.get(&current).and_then(|m| m.get(&keys))
    {
        return x
    }

    let result = keys.iter().filter_map(|&k|
    {
        let mut current = current.clone();
        let (d, p) =
        {
            let mut adj = None;
            for x in current.iter_mut()
            {
                if let Some((d, p)) = matrix.get(&(*x, k)).or_else(|| matrix.get(&(k, *x)))
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
            Some(d + collect_keys(current, keys, matrix, cache))
        }
    })
    .fold(u64::MAX, std::cmp::min);

    cache.entry(current).or_default().insert(keys, result);
    result
}
