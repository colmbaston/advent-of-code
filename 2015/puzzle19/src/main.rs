use std::collections::{ HashMap, HashSet };

fn main()
{
    let (grammar, molecule) = parse_grammar(include_str!("../input.txt"));

    println!("{}", replacements(&grammar, &molecule).len());
}

fn parse_grammar(s : &str) -> (HashMap<&str, Vec<&str>>, &str)
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

fn replacements(grammar : &HashMap<&str, Vec<&str>>, molecule : &str) -> HashSet<String>
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
