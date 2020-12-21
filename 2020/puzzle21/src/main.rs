use std::collections::{ HashSet, HashMap, hash_map::Entry };

fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_allergens).collect::<Vec<_>>();

    let mut allergens : HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, is) in input.iter().flat_map(|m| m.iter())
    {
        match allergens.entry(a)
        {
            Entry::Occupied(mut e) => { e.get_mut().retain(|i| is.contains(i)) },
            Entry::Vacant(e)       => { e.insert(is.clone()); }
        }
    }

    let mut solved = Vec::with_capacity(allergens.len());
    for _ in 0 .. allergens.len()
    {
        let (a, i) = allergens.iter()
                              .find_map(|(&a, is)| if is.len() == 1 { is.iter().next().map(|&i| (a, i)) } else { None })
                              .unwrap();

        for (_, is) in allergens.iter_mut() { is.remove(i); }
        solved.push((a, i));
    }

    println!("{}", input.iter()
                        .flat_map(|m| m.values().next().into_iter())
                        .flat_map(|s| s.iter())
                        .filter(|&i| !solved.iter().any(|(_, j)| i == j))
                        .count());

    solved.sort_unstable_by(|x, y| x.0.cmp(y.0));
    println!("{}", solved.into_iter()
                         .map(|(_, i)| i)
                         .collect::<Vec<_>>()
                         .join(","));
}

fn parse_allergens(s : &str) -> HashMap<&str, HashSet<&str>>
{
    let mut it = s[.. s.len()-1].split(" (contains ");

    let ingredients = it.next().unwrap().split_whitespace().collect::<HashSet<_>>();
    it.next().unwrap().split(", ").map(|allergen| (allergen, ingredients.clone())).collect()
}
