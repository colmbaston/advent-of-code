fn main()
{
    let mut root = Dir::new();
    root.parse_terminal(&mut include_str!("../input.txt").lines());

    let mut sizes = Vec::new();
    root.sizes(&mut sizes);

    println!("{}", sizes.iter().copied().filter(|&s| s <= 100000).sum::<u32>());

    let unused = 70000000 - root.size;
    println!("{}", sizes.iter().copied().filter(|&s| unused + s >= 30000000).min().unwrap_or(0));
}

struct Dir
{
    size:    u32,
    subtree: Vec<Dir>
}

impl Dir
{
    fn new() -> Dir
    {
        Dir { size: 0, subtree: Vec::new() }
    }

    fn parse_terminal<'a>(&mut self, lines : &mut impl Iterator<Item = &'a str>)
    {
        match lines.next()
        {
            None    => return,
            Some(l) =>
            {
                match l.split_whitespace().collect::<Vec<&str>>()[..]
                {
                    ["$", "cd", dir] => match dir
                    {
                        "/"  => (),
                        ".." => return,
                        _    =>
                        {
                            let mut subdir = Dir::new();
                            subdir.parse_terminal(lines);
                            self.size += subdir.size;
                            self.subtree.push(subdir);
                        }
                    },
                    ["$", "ls"]   => (),
                    ["dir", _dir] => (),
                    [size, _file] => self.size += size.parse::<u32>().unwrap_or(0),
                    _             => unreachable!()
                }
            }
        }

        self.parse_terminal(lines)
    }

    fn sizes(&self, sizes : &mut Vec<u32>)
    {
        sizes.push(self.size);
        self.subtree.iter().for_each(|subdir| subdir.sizes(sizes))
    }
}
