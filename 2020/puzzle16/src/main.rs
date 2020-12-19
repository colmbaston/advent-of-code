use std::ops::RangeInclusive;
use std::collections::{ HashSet, HashMap };

fn main()
{
    let mut input = parse_ticket_info(include_str!("../input.txt"));

    let mut error_rate = 0;
    let valid_values   = input.field_ranges.values().map(|r| r.iter().cloned().flatten()).flatten().collect::<HashSet<_>>();
    input.tickets.retain(|t| t.iter().fold(true, |v, k| if valid_values.contains(k) { v } else { error_rate += k; false }));
    println!("{}", error_rate);

    let mut possible = vec![input.field_ranges.keys().cloned().collect::<HashSet<_>>() ; input.field_ranges.len()];
    for t in input.tickets.iter()
    {
        for (k, p) in t.iter().zip(possible.iter_mut())
        {
            for (f, r) in input.field_ranges.iter()
            {
                if !r.iter().any(|r| r.contains(k)) { p.remove(f); }
            }
        }
    }

    let mut departure = 1;
    for _ in 0 .. possible.len()
    {
        let (i, f) = possible.iter()
                             .enumerate()
                             .find_map(|(i, p)| if p.len() == 1 { p.iter().next().map(|&f| (i, f)) } else { None })
                             .unwrap();

        for p in possible.iter_mut() { p.remove(f); }
        if f.starts_with("departure") { departure *= input.tickets[0][i] }
    }
    println!("{}", departure);
}

struct TicketInfo<'a>
{
    field_ranges: HashMap<&'a str, Vec<RangeInclusive<u64>>>,
    tickets:      Vec<Vec<u64>>
}

fn parse_ticket_info(s : &str) -> TicketInfo
{
    let mut it = s.split("\n\n");

    TicketInfo
    {
        field_ranges: it.next().unwrap()
                        .lines()
                        .map(parse_field_range)
                        .collect(),
        tickets:      it.next().unwrap()
                        .lines()
                        .skip(1)
                        .chain(it.next().unwrap().lines().skip(1))
                        .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
                        .collect()
    }
}

fn parse_field_range(s : &str) -> (&str, Vec<RangeInclusive<u64>>)
{
    let mut it = s.split(": ");

    (it.next().unwrap(),
     it.next().unwrap()
       .split(" or ")
       .map(|r| { let mut j = r.split('-'); j.next().unwrap().parse().unwrap() ..= j.next().unwrap().parse().unwrap() })
       .collect())
}
