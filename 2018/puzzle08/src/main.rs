fn main()
{
    let tree = Tree::parse(&mut include_str!("../input.txt")
                               .split_ascii_whitespace()
                               .map(|s| s.parse::<usize>().unwrap())).unwrap();

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
    fn parse(stream : &mut impl Iterator<Item = usize>) -> Option<Tree>
    {
        let c_count = stream.next()?;
        let m_count = stream.next()?;

        let mut children = Vec::with_capacity(c_count);
        for _ in 0 .. c_count
        {
            children.push(Tree::parse(stream)?)
        }

        let metadata = stream.take(m_count).collect();

        Some(Tree { children, metadata })
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
