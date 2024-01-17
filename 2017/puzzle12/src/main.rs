use std::collections::HashMap;

fn main()
{
    let mut graph = include_str!("../input.txt").lines()
                                                .map(parse_vertex)
                                                .collect::<HashMap<u32, Vec<u32>>>();

    println!("{}", purge(0, &mut graph));

    for k in 1 ..
    {
        match graph.keys().next()
        {
            None     => { println!("{k}"); break },
            Some(&v) => { purge(v, &mut graph);  }
        }
    }
}

fn parse_vertex(s : &str) -> (u32, Vec<u32>)
{
    let (a, b) = s.split_once(" <-> ").unwrap();
    (a.parse().unwrap(), b.split(", ").map(|k| k.parse().unwrap()).collect())
}

fn purge(vertex : u32, graph : &mut HashMap<u32, Vec<u32>>) -> u32
{
    match graph.remove(&vertex)
    {
        None      => 0,
        Some(adj) => 1 + adj.into_iter().map(|a| purge(a, graph)).sum::<u32>()
    }
}
