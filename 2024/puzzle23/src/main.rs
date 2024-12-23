use std::collections::{ HashMap, HashSet };

fn main()
{
    let graph = parse_graph(include_str!("../input.txt"));
    println!("{}", triangles(&graph).filter(|t| t.iter().any(|v| v.starts_with('t'))).count());
    let mut max_clique = bron_kerbosch(&graph).into_iter().collect::<Vec<&str>>();
    max_clique.sort_unstable();
    println!("{}", max_clique.join(","));
}

fn parse_graph(s : &str) -> HashMap<&str, HashSet<&str>>
{
    let mut graph = HashMap::new();
    for l in s.lines()
    {
        let (a, b) = l.split_once("-").unwrap();
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    graph
}

fn triangles<T>(graph : &HashMap<T, HashSet<T>>) -> impl Iterator<Item = [&T ; 3]> + '_
where T : Ord + std::hash::Hash
{
    graph.iter()
         .flat_map(move |(u, nu)| nu.iter()
                                    .filter(move |v| u < v)
                                    .flat_map(move |v| nu.intersection(&graph[v])
                                                         .filter(move |w| v < w)
                                                         .map(move |w| [u, v, w])))
}

fn bron_kerbosch<T>(graph : &HashMap<T, HashSet<T>>) -> HashSet<T>
where T : Copy + Eq + std::hash::Hash
{
    let mut max_clique = HashSet::new();
    let mut stack      = vec![(HashSet::new(),
                               graph.keys().copied().collect::<HashSet<T>>(),
                               HashSet::new())];

    while let Some((r, mut p, mut x)) = stack.pop()
    {
        if p.is_empty() && x.is_empty() && r.len() > max_clique.len()
        {
            max_clique = r.clone();
        }

        while let Some(&v) = p.iter().next()
        {
            let nv = &graph[&v];
            stack.push((r.iter().copied().chain(std::iter::once(v)).collect(),
                        p.intersection(nv).copied().collect(),
                        x.intersection(nv).copied().collect()));

            p.remove(&v);
            x.insert(v);
        }
    }

    max_clique
}
