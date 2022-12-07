fn main()
{
    let mut root = Dir::new();
    root.parse_terminal(&mut include_str!("../input.txt").lines());

    let mut sizes = Vec::new();
    root.sizes(&mut sizes);
    let free_space = 70000000 - root.size;

    let (one, two) = sizes.into_iter().fold((0, u32::MAX), |(one, two), size|
    {
        (if size              <= 100000   { one + size    } else { one },
         if free_space + size >= 30000000 { two.min(size) } else { two })
    });

    println!("{one}");
    println!("{two}");
}

struct Dir
{
    size:    u32,
    subdirs: Vec<Dir>
}

impl Dir
{
    fn new() -> Dir
    {
        Dir { size: 0, subdirs: Vec::new() }
    }

    fn parse_terminal<'a>(&mut self, lines : &mut impl Iterator<Item = &'a str>)
    {
        match lines.next()
        {
            None    => return,
            Some(l) =>
            {
                if let Some(comm) = l.strip_prefix("$ ")
                {
                    match comm.strip_prefix("cd ")
                    {
                        None | Some("/") => (),
                        Some("..")       => return,
                        Some(_)          =>
                        {
                            let mut subdir = Dir::new();
                            subdir.parse_terminal(lines);
                            self.size += subdir.size;
                            self.subdirs.push(subdir);
                        }
                    }
                }
                else
                {
                    let (digits, _) = l.split_at(l.find(|c : char| !c.is_ascii_digit()).unwrap_or(l.len()));
                    if let Ok(size) = digits.parse::<u32>() { self.size += size }
                }
            }
        }

        self.parse_terminal(lines)
    }

    fn sizes(&self, sizes : &mut Vec<u32>)
    {
        sizes.push(self.size);
        self.subdirs.iter().for_each(|subdir| subdir.sizes(sizes))
    }
}
