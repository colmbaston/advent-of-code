fn main()
{
    let nodes = include_str!("../input.txt").lines().skip(2).map(Node::parse).collect::<Vec<Node>>();

    let mut count = 0;
    for na in nodes.iter()
    {
        for nb in nodes.iter()
        {
            if !na.is_empty()
            && na.pos  != nb.pos
            && na.used <= nb.avail() { count += 1 }
        }
    }
    println!("{count}");
}

type Pos = (u32, u32);

struct Node
{
    pos: Pos,
    size: u32,
    used: u32
}

impl Node
{
    fn parse(s : &str) -> Node
    {
        let        s  = s.strip_prefix("/dev/grid/node-x").unwrap();
        let (x,    s) = s.split_once("-y").unwrap();
        let (y,    s) = s.split_once(' ').unwrap();
        let (size, s) = s.trim_start().split_once('T').unwrap();
        let (used, _) = s.trim_start().split_once('T').unwrap();

        Node
        {
            pos:  (x.parse().unwrap(), y.parse().unwrap()),
            size: size.parse().unwrap(),
            used: used.parse().unwrap()
        }
    }

    fn is_empty(&self) -> bool
    {
        self.used == 0
    }

    fn avail(&self) -> u32
    {
        self.size - self.used
    }
}
