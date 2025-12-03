use std::{ hash::Hash, collections::HashMap };

fn main()
{
    let graph       = parse_graph(include_str!("../input.txt"));
    let mut mapping = HashMap::new();

    loop
    {
        let mut graph = graph.clone();
        mapping.clear();
        karger(&mut graph, &mut mapping);

        if graph.values().next().and_then(|v| v.values().next()) == Some(&3) { break }
    }

    let mut partition = HashMap::new();
    for mut k in graph.keys()
    {
        while let Some(l) = mapping.get(k) { k = l }
        *partition.entry(k).or_insert(0) += 1;
    }
    println!("{}", partition.values().product::<u32>());
}

type Graph<T> = HashMap<T, HashMap<T, u32>>;

fn parse_graph(s : &str) -> Graph<&str>
{
    let mut graph = HashMap::new();
    for l in s.lines()
    {
        let (u, vs) = l.split_once(':').unwrap();
        for v in vs.split_whitespace()
        {
            *graph.entry(u).or_insert_with(HashMap::new).entry(v).or_insert(0) += 1;
            *graph.entry(v).or_insert_with(HashMap::new).entry(u).or_insert(0) += 1;
        }
    }
    graph
}

fn karger<T : Copy + Eq + Hash>(graph : &mut Graph<T>, mapping : &mut HashMap<T, T>)
{
    use rand::seq::{ IteratorRandom, IndexedRandom };
    let mut rng = rand::rng();

    for _ in 2 .. graph.len()
    {
        let (&u, vs) = graph.iter().choose(&mut rng).unwrap();
        let  &v      = vs.iter().collect::<Vec<_>>().choose_weighted(&mut rng, |&(_, &k)| k).unwrap().0;

        contract(u, v, graph);
        mapping.insert(v, u);
    }
}

fn contract<T : Copy + Eq + Hash>(u : T, v: T, graph : &mut Graph<T>)
{
    graph.get_mut(&u).unwrap().remove(&v);
    let mut v_adj = graph.remove(&v).unwrap();
    v_adj.remove(&u);

    for (w, j) in v_adj.into_iter()
    {
        *graph.entry(u).or_default().entry(w).or_insert(0) += j;
        let w_adj = graph.entry(w).or_default();
        if let Some(k) = w_adj.remove(&v) { *w_adj.entry(u).or_insert(0) += k }
    }
}
