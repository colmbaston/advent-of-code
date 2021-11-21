use std::collections::{ HashMap, HashSet };

fn main()
{
    let (grammar, molecule) = parse_grammar(include_str!("../input.txt"));

    println!("{}", replacements(&grammar, molecule).len());
    println!("{}", greedy(&transpose(&grammar), molecule));
}

type Grammar<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_grammar(s : &str) -> (Grammar, &str)
{
    let mut i       = s.split("\n\n");
    let mut grammar = HashMap::new();

    for l in i.next().unwrap().lines()
    {
        let mut j = l.split(" => ");
        grammar.entry(j.next().unwrap()).or_insert_with(Vec::new).push(j.next().unwrap())
    }

    (grammar, i.next().unwrap().trim_end())
}

fn replacements(grammar : &Grammar, molecule : &str) -> HashSet<String>
{
    let mut res = HashSet::new();
    for (a, bs) in grammar.iter()
    {
        for b in bs.iter()
        {
            for k in 0 .. molecule.len()
            {
                if let Some(c) = molecule[k ..].strip_prefix(a)
                {
                    let mut s = String::with_capacity(k + b.len() + c.len());
                    s.push_str(&molecule[.. k]);
                    s.push_str(b);
                    s.push_str(c);
                    res.insert(s);
                }
            }
        }
    }
    res
}

fn transpose<'a>(grammar : &Grammar<'a>) -> Grammar<'a>
{
    let mut res = HashMap::new();

    for (&s, v) in grammar.iter()
    {
        for &t in v.iter()
        {
            res.entry(t).or_insert_with(Vec::new).push(s)
        }
    }

    res
}

fn greedy(grammar : &Grammar, molecule : &str) -> u32
{
    'reset: loop
    {
        let mut current = molecule.to_string();
        let mut steps   = 0;

        while current != "e"
        {
            match replacements(grammar, &current).into_iter().next()
            {
                Some(next) => { current = next; steps += 1 }
                None       => continue 'reset
            }
        }

        return steps
    }
}
