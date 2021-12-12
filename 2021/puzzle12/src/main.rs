use std::collections::{ HashMap, HashSet };

fn main()
{
    let input = parse_graph(include_str!("../input.txt"));

    println!("{}", explore("start", &input, HashSet::new(), true ).count());
    println!("{}", explore("start", &input, HashSet::new(), false).count());
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

fn explore<'a>(pos : &'a str, graph : &'a HashMap<&'a str, Vec<&'a str>>, mut visited : HashSet<&'a str>, mut small : bool) -> Box<dyn Iterator<Item = Vec<&'a str>> + 'a>
{
    if pos == "end" { return Box::new(std::iter::once(vec![pos])) }

    if pos.bytes().next().unwrap().is_ascii_lowercase() && !visited.insert(pos)
    {
        if small || pos == "start"
        {
            return Box::new(std::iter::empty());
        }
        else
        {
            small = true;
        }
    }

    Box::new(graph.get(pos).unwrap().iter()
                  .flat_map(move |q| explore(q, graph, visited.clone(), small))
                  .map(|mut v| { v.push(pos); v }))
}
