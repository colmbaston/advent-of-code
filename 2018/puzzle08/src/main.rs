fn main()
{
    let (_, tree) = Tree::parse(&include_str!("../input.txt").split_ascii_whitespace()
                                                             .map(|s| s.parse::<usize>().unwrap())
                                                             .collect::<Vec<usize>>());

    println!("{}", tree.sum_metadata());
    println!("{}", tree.root_value());
}

struct Tree
{
    children: Vec<Tree>,
    metadata: Vec<usize>
}

impl Tree
{
    fn parse(s : &[usize]) -> (&[usize], Tree)
    {
        let c_count = s[0];
        let m_count = s[1];
        let mut children = Vec::with_capacity(c_count);
        let mut metadata = Vec::with_capacity(m_count);
        let mut s = &s[2..];

        for _ in 0 .. c_count
        {
            let (r, t) = Tree::parse(s);
            children.push(t);
            s = r;
        }

        let (ms, s) = s.split_at(m_count);
        metadata.extend(ms.iter().copied());

        (s, Tree { children, metadata })
    }

    fn sum_metadata(&self) -> usize
    {
          self.metadata.iter().sum::<usize>()
        + self.children.iter().map(|t| t.sum_metadata()).sum::<usize>()
    }

    fn root_value(&self) -> usize
    {
        if self.children.is_empty()
        {
            self.sum_metadata()
        }
        else
        {
            self.metadata.iter().map(|&i|
            {
                if 1 <= i && i <= self.children.len()
                {
                    self.children[i - 1].root_value()
                }
                else
                {
                    0
                }
            })
            .sum()
        }
    }
}
