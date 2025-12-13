use std::collections::HashMap;

fn main()
{
    let mut nodes = include_str!("../input.txt").lines().map(parse_node).collect::<HashMap<&str, (i32, Vec<&str>)>>();
    let mut trees = HashMap::new();

    while let Some(&name) = nodes.iter().find(|(_, (_, cs))| cs.iter().all(|c| trees.contains_key(c))).map(|(name, _)| name)
    {
        let (weight, children) = nodes.remove(name).unwrap();
        let          children  = children.into_iter().flat_map(|c| trees.remove(&c)).collect::<Vec<Tree>>();
        trees.insert(name, Tree { weight, children });
    }

    let (name, mut tree) = trees.into_iter().next().unwrap();
    println!("{name}");

    tree.propagate();
    loop
    {
        if tree.children.len() == 1
        {
            tree = tree.children.into_iter().next().unwrap();
        }
        else
        {
            match tree.children.iter().position(|t| !t.balanced())
            {
                None    => break,
                Some(i) => tree = tree.children.into_iter().nth(i).unwrap()
            }
        }
    }

    tree.children.sort_by_key(|t| t.weight);
    let [a, b] = tree.children.array_windows().next().unwrap();
    let diff = if a.weight != b.weight
    {
        let diff = b.weight - a.weight;
        tree = tree.children.into_iter().next().unwrap();
        diff
    }
    else
    {
        let [a, b] = tree.children.array_windows().last().unwrap();
        let diff = a.weight - b.weight;
        tree = tree.children.into_iter().last().unwrap();
        diff
    };

    tree.unpropagate();
    println!("{}", tree.weight + diff);
}

struct Tree
{
    weight:   i32,
    children: Vec<Tree>
}

impl Tree
{
    fn propagate(&mut self)
    {
        self.children.iter_mut().for_each(Tree::propagate);
        self.weight += self.children.iter().map(|c| c.weight).sum::<i32>()
    }

    fn balanced(&self) -> bool
    {
        match self.children.split_first()
        {
            None                => true,
            Some((first, rest)) => rest.iter().all(|c| c.weight == first.weight)
        }
    }

    fn unpropagate(&mut self)
    {
        self.weight -= self.children.iter().map(|c| c.weight).sum::<i32>();
        self.children.iter_mut().for_each(Tree::unpropagate)
    }
}

fn parse_node(s : &str) -> (&str, (i32, Vec<&str>))
{
    let (name,   s) = s.split_once(' ').unwrap();
    let (weight, s) = s.strip_prefix('(').unwrap().split_once(')').unwrap();
    let children    = s.strip_prefix(" -> ").into_iter()
                                            .flat_map(|s| s.split(", "))
                                            .collect::<Vec<&str>>();

    (name, (weight.parse().unwrap(), children))
}
