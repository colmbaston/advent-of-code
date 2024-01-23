use std::collections::HashSet;

fn main()
{
    let comps       = include_str!("../input.txt").lines().map(Bridge::parse_comp).collect::<Vec<(u32, u32)>>();
    let mut current = Bridge(Vec::new());
    let mut visited = HashSet::new();
    let mut bridges = Vec::new();
    current.dfs(&comps, &mut visited, &mut bridges);

    println!("{}", bridges.iter().map(|b|             b.strength() ).max().unwrap());
    println!("{}", bridges.iter().map(|b| (b.0.len(), b.strength())).max().unwrap().1);
}

#[derive(Clone)]
struct Bridge(Vec<Comp>);
type Comp = (u32, u32);

impl Bridge
{
    fn parse_comp(s : &str) -> Comp
    {
        let (a, b) = s.split_once('/').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }

    fn strength(&self) -> u32
    {
        self.0.iter().flat_map(|(l, r)| [l, r]).sum()
    }

    fn dfs(&mut self, comps : &[Comp], visited : &mut HashSet<usize>, bridges : &mut Vec<Bridge>)
    {
        let port = self.0.last().map(|(_, r)| *r).unwrap_or(0);
        for (i, (l, r)) in comps.iter().copied().enumerate()
        {
            if l != port && r != port || !visited.insert(i) { continue }
            self.0.push(if l == port { (l, r) } else { (r, l) });
            bridges.push(self.clone());
            self.dfs(comps, visited, bridges);
            self.0.pop();
            visited.remove(&i);
        }
    }
}
