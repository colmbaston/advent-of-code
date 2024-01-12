fn main()
{
    let nodes  = include_str!("../input.txt").lines().skip(2).map(Node::parse).collect::<Vec<Node>>();
    let height = nodes.iter().take_while(|n| n.pos.0 == 0).count();
    let width  = nodes.len() / height;

    let mut count = 0;
    for (i, na) in nodes.iter().enumerate()
    {
        for nb in nodes.iter().take(i).chain(nodes.iter().skip(i+1))
        {
            if !na.is_empty() && na.used <= nb.avail() { count += 1 }
        }
    }
    println!("{count}");

    // part two makes several simplifying assumptions which don't hold in general
    // assume exactly one empty position which is used for every data transfer
    let (ex, ey) = nodes.iter().find(|n| n.is_empty()).unwrap().pos;

    // nearest open column to empty node
    // a column is open when it contains no blocking nodes between the empty node and y=0
    // a blocking node contains so much data that it can never transfer it to its neighbours
    let open = (0 .. width).filter(|x| (1 .. ey).all(|y| nodes[y + x * height].used <= nodes[y-1 + x * height].size))
                           .min_by_key(|x| x.abs_diff(ex))
                           .unwrap();

    println!("{}", ex.abs_diff(open)                                        // moving the empty node to the nearest open column
                 + ey                                                       // moving the empty node to y=0
                 + if open == width-1 { 2 } else { open.abs_diff(width-1) } // moving the empty node to the goal at the max x, shifting the goal left
                 + 5 * (width - 2))                                         // moving the goal to x=0, each shift needing five moves of the empty node
}

type Pos = (usize, usize);

struct Node
{
    pos:  Pos,
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
