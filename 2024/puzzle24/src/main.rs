use std::collections::HashMap;

fn main()
{
    let graph  = include_str!("../input.txt").split("\n\n").flat_map(|s| s.lines()).map(Expr::parse).collect::<HashMap<&str, Expr>>();
    let mut zs = graph.keys().copied().filter(|k| k.starts_with('z')).collect::<Vec<&str>>();
    zs.sort_unstable();
    println!("{}", zs.iter().rev().fold(0, |a, z| a << 1 | graph[z].eval(&graph) as u64));

    // part two answer hard-coded as it was found by manual inspection of the input
    // the individual swaps are (vcw, z13), (vwp, z19), (mps, z25), (cqm, vjv)
    println!("cqm,mps,vcv,vjv,vwp,z13,z19,z25");
}

enum Expr<'a>
{
    Const(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str)
}

impl Expr<'_>
{
    fn parse(s : &str) -> (&str, Expr)
    {
        match s.split_once(": ")
        {
            Some((a, b)) => (a, Expr::Const(b == "1")),
            None         =>
            {
                let (a, b) = s.split_once(" -> ").unwrap();
                (b, match a.split_whitespace().collect::<Vec<&str>>()[..]
                {
                    [c, "AND", d] => Expr::And(c, d),
                    [c, "OR",  d] => Expr::Or(c, d),
                    [c, "XOR", d] => Expr::Xor(c, d),
                    _             => unreachable!()
                })
            }
        }
    }

    fn eval(&self, graph : &HashMap<&str, Expr>) -> bool
    {
        match self
        {
            Expr::Const(b)  => *b,
            Expr::And(a, b) => graph[a].eval(graph) && graph[b].eval(graph),
            Expr::Or(a, b)  => graph[a].eval(graph) || graph[b].eval(graph),
            Expr::Xor(a, b) => graph[a].eval(graph) != graph[b].eval(graph)
        }
    }
}
