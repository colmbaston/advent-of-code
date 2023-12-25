use std::{ hash::Hash, collections::{ HashMap, HashSet, VecDeque }};

fn main()
{
    let mut graph = parse_graph(include_str!("../input.txt"));
    let mut hist  = HashMap::new();

    for &u in graph.keys()
    {
        let mut visited = HashSet::new();
        let mut queue   = VecDeque::new();
        queue.push_back(u);

        while let Some(v) = queue.pop_front()
        {
            for &w in graph[v].iter()
            {
                if !visited.insert(w) { continue }

                *hist.entry((v.min(w), v.max(w))).or_insert(0) += 1;
                queue.push_back(w)
            }
        }
    }

    let mut hist = hist.into_iter().collect::<Vec<((&str, &str), usize)>>();
    hist.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));

    for ((u, v), _) in hist.into_iter().take(3)
    {
        graph.entry(u).and_modify(|ws| { ws.remove(v); });
        graph.entry(v).and_modify(|ws| { ws.remove(u); });
    }

    println!("{}", connected_components(&graph).product::<usize>());
}

type Graph<T> = HashMap<T, HashSet<T>>;

fn parse_graph(s : &str) -> Graph<&str>
{
    let mut graph = HashMap::new();
    for l in s.lines()
    {
        let (u, vs) = l.split_once(": ").unwrap();
        for v in vs.split_whitespace()
        {
            graph.entry(u).or_insert_with(HashSet::new).insert(v);
            graph.entry(v).or_insert_with(HashSet::new).insert(u);
        }
    }
    graph
}

fn connected_components<T : Copy + Eq + Hash>(graph : &Graph<T>) -> impl Iterator<Item = usize> + '_
{
    let mut visited = HashSet::new();
    graph.keys()
         .map(move |&vertex| dfs(vertex, graph, &mut visited))
         .filter(|&size| size > 0)
}

fn dfs<T : Copy + Eq + Hash>(vertex : T, graph : &Graph<T>, visited : &mut HashSet<T>) -> usize
{
    if !visited.insert(vertex) { return 0 }

    1 + graph[&vertex].iter().map(|&v| dfs(v, graph, visited)).sum::<usize>()
}
