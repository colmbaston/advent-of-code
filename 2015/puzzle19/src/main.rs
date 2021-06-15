use std::collections::HashSet;

fn main()
{
    let (rs, molecule) = parse(include_str!("../input.txt"));

    println!("{}", replacements(&rs, &molecule).len());
}

fn parse(s : &str) -> (Vec<(&str, &str)>, &str)
{
    let mut i = s.split("\n\n");

    (i.next().unwrap().lines().map(|s| { let mut j = s.split(" => "); (j.next().unwrap(), j.next().unwrap()) }).collect(),
     i.next().unwrap().trim_end())
}

fn replacements(rs : &[(&str, &str)], molecule : &str) -> HashSet<String>
{
    let mut s = HashSet::new();
    for (a, b) in rs.iter()
    {
        for k in 0 .. molecule.len()
        {
            if molecule[k ..].starts_with(a)
            {
                let mut v = molecule[.. k].to_string();
                v.push_str(b);
                v.push_str(&molecule[k + a.len() ..]);
                s.insert(v);
            }
        }
    }
    s
}
