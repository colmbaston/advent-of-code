fn main()
{
    let games = include_str!("../input.txt").lines().map(|l| l.split_once(": ").unwrap().1
                                                              .split("; ")
                                                              .map(Subset::parse)
                                                              .collect::<Vec<Subset>>())
                                                    .collect::<Vec<Vec<Subset>>>();

    println!("{}", games.iter().zip(1..)
                        .filter(|(g, _)| g.iter().all(|&set| Subset { r: 12, g: 13, b: 14 }.contains(set)))
                        .map(|(_, i)| i)
                        .sum::<u32>());

    println!("{}", games.iter()
                        .map(|g| g.iter().copied().fold(Subset::zero(), Subset::saturate).power())
                        .sum::<u32>());
}

#[derive(Copy, Clone)]
struct Subset
{
    r: u32,
    g: u32,
    b: u32
}

impl Subset
{
    fn zero() -> Subset
    {
        Subset { r: 0, g: 0, b:  0 }
    }

    fn contains(self, other : Subset) -> bool
    {
        other.r <= self.r &&
        other.g <= self.g &&
        other.b <= self.b
    }

    fn saturate(self, other : Subset) -> Subset
    {
        Subset
        {
            r: self.r.max(other.r),
            g: self.g.max(other.g),
            b: self.b.max(other.b)
        }
    }

    fn power(self) -> u32
    {
        self.r * self.g * self.b
    }

    fn parse(s : &str) -> Subset
    {
        let mut set = Subset::zero();
        for s in s.split(", ")
        {
            let (count, colour) = s.split_once(' ').unwrap();

            let field = match colour
            {
                "red"   => &mut set.r,
                "green" => &mut set.g,
                "blue"  => &mut set.b,
                _       => unreachable!()
            };

            *field += count.parse::<u32>().unwrap();
        }
        set
    }
}
