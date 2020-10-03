use std::collections::{ BTreeMap, HashSet };

fn main()
{
    // build the dependency graph using a BTreeMap
    // since the ordering of the keys will matter
    let mut graph = BTreeMap::new();
    for c in b'A' ..= b'Z'
    {
        graph.insert(c, HashSet::new());
    }
    for s in include_str!("../input.txt").lines().map(|s| s.as_bytes())
    {
        graph.get_mut(&s[36]).unwrap().insert(s[5]);
    }

    // find and print the first step with no dependencies, removing it
    // from the graph and removing it as a dependency for the other steps
    while let Some((&k, _)) = graph.iter().find(|(_, v)| v.is_empty())
    {

        print!("{}", k as char);
        graph.remove(&k);
        for (_, v) in graph.iter_mut()
        {
            v.remove(&k);
        }
    }
    println!();
}
