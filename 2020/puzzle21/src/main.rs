use std::collections::{ HashSet, HashMap, hash_map::Entry };

fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_allergens).collect::<Vec<_>>();

    let mut possible : HashMap<&str, HashSet<&str>> = HashMap::new();
    for (is, als) in input.iter()
    {
        let is = is.iter().cloned().collect::<HashSet<_>>();
        for a in als.iter()
        {
            match possible.entry(a)
            {
                Entry::Occupied(mut e) => { e.get_mut().retain(|i| is.contains(i)) },
                Entry::Vacant(e)       => { e.insert(is.clone()); }
            }
        }
    }

    let mut solved = Vec::with_capacity(possible.len());
    for _ in 0 .. possible.len()
    {
        let (a, i) = possible.iter()
                             .find_map(|(&a, is)| if is.len() == 1 { is.iter().next().map(|&i| (a, i)) } else { None })
                             .unwrap();

        possible.remove(a);
        for (_, is) in possible.iter_mut() { is.remove(i); }
        solved.push((a, i));
    }

    let allergenic = solved.iter().map(|(_, i)| i).collect::<HashSet<_>>();
    println!("{}", input.iter()
                        .flat_map(|(is, _)| is.iter())
                        .filter(|i| !allergenic.contains(i))
                        .count());

    solved.sort_unstable_by(|x, y| x.0.cmp(y.0));
    println!("{}", solved.into_iter()
                         .map(|(_, i)| i)
                         .collect::<Vec<_>>()
                         .join(","));
}

fn parse_allergens(s : &str) -> (Vec<&str>, Vec<&str>)
{
    let mut it = s[.. s.len()-1].split(" (contains ");

    (it.next().unwrap().split_whitespace().collect(),
     it.next().unwrap().split(", ").collect())
}
