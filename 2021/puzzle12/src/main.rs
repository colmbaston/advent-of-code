use std::collections::{ HashMap, HashSet };

fn main()
{
    let input = parse_graph(include_str!("../input.txt"));

    println!("{}", explore("start", &input, HashSet::new(), true));
    println!("{}", explore("start", &input, HashSet::new(), false));
}

fn parse_graph(s : &str) -> HashMap<&str, Vec<&str>>
{
    let mut graph = HashMap::new();
    for l in s.lines()
    {
        match l.split('-').collect::<Vec<_>>()[..]
        {
            [a, b] =>
            {
                graph.entry(a).or_insert_with(Vec::new).push(b);
                graph.entry(b).or_insert_with(Vec::new).push(a);
            }
            _ => unreachable!()
        }
    }
    graph
}

fn explore<'a>(pos : &'a str, graph : &HashMap<&str, Vec<&str>>, mut visited : HashSet<&'a str>, mut small : bool) -> u32
{
    if pos == "end" { return 1 }

    if pos.bytes().next().unwrap().is_ascii_lowercase() && !visited.insert(pos)
    {
        if small || pos == "start"
        {
            return 0;
        }
        else
        {
            small = true;
        }
    }

    graph.get(pos).unwrap().iter()
         .map(|q| explore(q, graph, visited.clone(), small))
         .sum()
}
