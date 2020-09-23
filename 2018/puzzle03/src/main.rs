use std::collections::{ HashSet, HashMap, hash_map::Entry };

fn main()
{
    let input = include_str!("../input.txt").lines().map(|s| parse_claim(s).unwrap().1).collect::<Vec<_>>();

    let mut areas        = HashMap::new();
    let mut intersecting = HashSet::new();

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
                        intersecting.insert(id);
                        intersecting.insert(*other);
                        *intersects = true;
                    }
                }
            }
        }
    }

    println!("{}", areas.values().fold(0, |a, &(_, x)| a + x as u32));
    println!("{}", input.iter().map(|c| c.id).collect::<HashSet<_>>().difference(&intersecting).next().unwrap());
}

#[derive(PartialEq, Eq, Hash)]
struct Claim
{
    id:        u32,
    position: (u32, u32),
    area:     (u32, u32)
}

fn parse_claim(s : &str) -> nom::IResult<&str, Claim>
{
    use nom::character::complete::digit1;

    let (s, id) = digit1(&s[1..])?;
    let (s, x)  = digit1(&s[3..])?;
    let (s, y)  = digit1(&s[1..])?;
    let (s, w)  = digit1(&s[2..])?;
    let (s, h)  = digit1(&s[1..])?;

    Ok((s, Claim
    {
        id:       id.parse().unwrap(),
        position: (x.parse().unwrap(), y.parse().unwrap()),
        area:     (w.parse().unwrap(), h.parse().unwrap())
    }))
}
