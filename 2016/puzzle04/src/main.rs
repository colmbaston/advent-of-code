use std::collections::HashMap;

fn main()
{
    let rooms = include_str!("../input.txt").lines()
                                            .map(Room::parse)
                                            .collect::<Vec<Room>>();

    println!("{}", rooms.iter()
                        .filter(|r| r.real())
                        .map(|r| r.sector)
                        .sum::<u32>());

    println!("{}", rooms.iter()
                        .find(|r| r.decrypt() == ["northpole", "object", "storage"])
                        .unwrap()
                        .sector);
}

struct Room<'a>
{
    name:     Vec<&'a str>,
    sector:   u32,
    checksum: &'a str
}

impl<'a> Room<'a>
{
    fn parse(s : &str) -> Room
    {
        let mut name = s.split('-').collect::<Vec<&str>>();
        let (sector, checksum) = name.pop().unwrap().split_once('[').unwrap();

        Room
        {
            name,
            sector:   sector.parse().unwrap(),
            checksum: checksum.strip_suffix(']').unwrap()
        }
    }

    fn real(&self) -> bool
    {
        let mut hist = HashMap::new();
        for c in self.name.iter().flat_map(|word| word.chars())
        {
            *hist.entry(c).or_insert(0) += 1
        }

        let mut hist = hist.into_iter().collect::<Vec<(char, u32)>>();
        hist.sort_unstable_by(|(c1, f1), (c2, f2)| f2.cmp(f1).then(c1.cmp(c2)));

        self.checksum.chars()
                     .zip(hist.into_iter().map(|(c, _)| c))
                     .all(|(a, b)| a == b)
    }

    fn decrypt(&self) -> Vec<String>
    {
        self.name.iter()
                 .map(|word| word.chars()
                                 .map(|c| char::from_u32(((c as u32 - 'a' as u32 + self.sector) % 26) + 'a' as u32).unwrap())
                                 .collect())
                 .collect()
    }
}
