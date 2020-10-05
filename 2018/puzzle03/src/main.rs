use std::collections::{ HashSet, HashMap, hash_map::Entry };

fn main()
{
    let input = include_str!("../input.txt").lines().map(|s| parse_claim(s)).collect::<Vec<_>>();

    let mut areas       = HashMap::new();
    let mut uncontested = input.iter().map(|c| c.id).collect::<HashSet<_>>();
    for &Claim { id, position: (x, y), area: (w, h) } in input.iter()
    {
        for i in x .. x+w
        {
            for j in y .. y+h
            {
                match areas.entry((i, j))
                {
                    Entry::Vacant(e) =>
                    {
                        e.insert((id, false));
                    },
                    Entry::Occupied(mut e) =>
                    {
                        let (other, intersects) = e.get_mut();
                        uncontested.remove(&id);
                        uncontested.remove(other);
                        *intersects = true;
                    }
                }
            }
        }
    }

    println!("{}", areas.values().fold(0, |a, &(_, x)| a + x as u32));
    println!("{}", uncontested.iter().next().unwrap());
}

#[derive(PartialEq, Eq, Hash)]
struct Claim
{
    id:        u32,
    position: (u32, u32),
    area:     (u32, u32)
}

fn parse_claim(s : &str) -> Claim
{
    fn span_digits(s : &str) -> (&str, &str)
    {
        s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or_else(|| s.len()))
    }

    let (id, s) = span_digits(&s[1..]);
    let (x,  s) = span_digits(&s[3..]);
    let (y,  s) = span_digits(&s[1..]);
    let (w,  s) = span_digits(&s[2..]);
    let (h,  _) = span_digits(&s[1..]);

    Claim
    {
        id:       id.parse().unwrap(),
        position: (x.parse().unwrap(), y.parse().unwrap()),
        area:     (w.parse().unwrap(), h.parse().unwrap())
    }
}
