use std::collections::HashMap;

fn main()
{
    let (dirs, graph) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let        graph  = parse_graph(graph);

    println!("{}", steps(dirs, &graph, "AAA", |node| node == "ZZZ"));
    println!("{}", graph.keys()
                        .filter(|node| node.as_bytes().last() == Some(&b'A'))
                        .map(|start| steps(dirs, &graph, start, |node| node.as_bytes().last() == Some(&b'Z')))
                        .fold(1, lcm));
}

fn parse_graph(s : &str) -> HashMap<&str, (&str, &str)>
{
    s.lines().map(|l|
    {
        let (source, rest) = l.split_once(" = (").unwrap();
        let (left, right)  = rest.strip_suffix(')').unwrap().split_once(", ").unwrap();

        (source, (left, right))
    })
    .collect()
}

fn steps<'a>(dirs : &str, graph : &HashMap<&'a str, (&'a str, &'a str)>, start : &'a str, end : impl Fn(&'a str) -> bool) -> u64
{
    dirs.bytes().cycle().zip(0..).try_fold(start, |node, (dir, step)|
    {
        if end(node)
        {
            Err(step)
        }
        else
        {
            let &(left, right) = graph.get(node).unwrap();
            Ok(if let b'L' = dir { left } else { right })
        }
    })
    .unwrap_err()
}

fn lcm(a : u64, b : u64) -> u64
{
    (a * b) / gcd(a, b)
}

fn gcd(a : u64, b : u64) -> u64
{
    if let 0 = a { b } else { gcd(b % a, a) }
}
