use std::collections::HashMap;

fn main()
{
    let (mut rules, input) = parse_input(include_str!("../input.txt"));

    println!("{}", input.iter().filter(|src| rules.get(&0).unwrap().full_match(src, &rules)).count());

    rules.insert(8,  Rule::SubRules(vec![vec![42],     vec![42, 8]]));
    rules.insert(11, Rule::SubRules(vec![vec![42, 31], vec![42, 11, 31]]));
    println!("{}", input.iter().filter(|src| rules.get(&0).unwrap().full_match(src, &rules)).count());
}

enum Rule
{
    Char(char),
    SubRules(Vec<Vec<u32>>)
}

fn parse_input(s : &str) -> (HashMap<u32, Rule>, Vec<&str>)
{
    let mut it = s.split("\n\n");

    (it.next().unwrap().lines().map(Rule::parse).collect(),
     it.next().unwrap().lines().collect())
}

impl Rule
{
    fn parse(s : &str) -> (u32, Rule)
    {
        let mut it = s.split(": ");
        let id = it.next().unwrap().parse().unwrap();
        let s  = it.next().unwrap();

        (id, match s.strip_prefix('"')
        {
            None    => Rule::SubRules(s.split('|').map(|r| r.split_whitespace().map(|x| x.parse().unwrap()).collect()).collect()),
            Some(s) => Rule::Char(s.chars().next().unwrap())
        })
    }

    fn full_match(&self, src : &str, rules : &HashMap<u32, Rule>) -> bool
    {
        self.prefix_match(src, rules).contains(&"")
    }

    fn prefix_match<'a>(&self, src : &'a str, rules : &HashMap<u32, Rule>) -> Vec<&'a str>
    {
        match self
        {
            Rule::Char(c)      => src.strip_prefix(|d| *c == d).into_iter().collect(),
            Rule::SubRules(rs) => rs.iter().flat_map(|alt| alt.iter().fold(vec![src], |v, id|
            {
                let r = rules.get(id).unwrap();
                v.into_iter().flat_map(|s| r.prefix_match(s, rules).into_iter()).collect()
            }))
            .collect()
        }
    }
}
