use std::collections::HashMap;

fn main()
{
    let (mut rules, strings) = parse_input(include_str!("../input.txt"));

    println!("{}", strings.iter().filter(|src| rules.get(&0).unwrap().full_match(src, &rules)).count());

    rules.insert(8,  Rule::SubRules(vec![vec![42],     vec![42, 8]]));
    rules.insert(11, Rule::SubRules(vec![vec![42, 31], vec![42, 11, 31]]));
    println!("{}", strings.iter().filter(|src| rules.get(&0).unwrap().full_match(src, &rules)).count());
}

fn parse_input(s : &str) -> (HashMap<u32, Rule>, Vec<&str>)
{
    let mut i = s.split("\n\n");

    (i.next().unwrap().lines().map(Rule::parse).collect(),
     i.next().unwrap().lines().collect())
}

enum Rule
{
    Char(char),
    SubRules(Vec<Vec<u32>>)
}

impl Rule
{
    fn parse(s : &str) -> (u32, Rule)
    {
        let mut i = s.split(": ");
        let id = i.next().unwrap().parse().unwrap();
        let s  = i.next().unwrap();

        (id, match s.strip_prefix('"')
        {
            None    => Rule::SubRules(s.split('|').map(|r| r.split_whitespace().map(|x| x.parse().unwrap()).collect()).collect()),
            Some(s) => Rule::Char(s.chars().next().unwrap())
        })
    }

    fn full_match(&self, src : &str, rules : &HashMap<u32, Rule>) -> bool
    {
        self.prefix_match(src, rules).into_iter().any(|s| s.is_empty())
    }

    fn prefix_match<'a>(&self, src : &'a str, rules : &HashMap<u32, Rule>) -> Vec<&'a str>
    {
        match self
        {
            Rule::Char(c)      => src.strip_prefix(|d| *c == d).into_iter().collect(),
            Rule::SubRules(rs) => rs.iter().flat_map(|alt|
            {
                let mut t = vec![src];
                for id in alt.iter()
                {
                    let r = rules.get(&id).unwrap();
                    t = t.into_iter().flat_map(|s| r.prefix_match(s, rules).into_iter()).collect();
                }
                t.into_iter()
            })
            .collect()
        }
    }
}
