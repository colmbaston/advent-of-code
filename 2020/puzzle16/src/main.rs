use std::ops::RangeInclusive;
use std::collections::{ HashSet, HashMap };

fn main()
{
    let mut input = parse_ticket_info(include_str!("../input.txt"));

    let mut error_rate = 0;
    let ranges    = input.field_ranges.values().map(|r| r.iter().cloned().flatten()).flatten().collect::<HashSet<_>>();
    input.tickets = input.tickets.into_iter()
                                 .filter(|t| t.iter().fold(true, |v, k| if ranges.contains(k) { v } else { error_rate += k; false }))
                                 .collect();
    println!("{}", error_rate);

    let mut possible = vec![input.field_ranges.keys().cloned().collect::<HashSet<_>>() ; input.field_ranges.len()];
    for t in input.tickets.iter()
    {
        for (k, p) in t.iter().zip(possible.iter_mut())
        {
            for (f, r) in input.field_ranges.iter()
            {
                if !r.iter().any(|r| r.contains(k))
                {
                    p.remove(f);
                }
            }
        }
    }

    let mut departure = 1;
    while !possible.iter().all(|p| p.is_empty())
    {
        let (i, s) = possible.iter().enumerate().find(|(_, p)| p.len() == 1).unwrap();
        let s = *s.iter().next().unwrap();
        possible.iter_mut().for_each(|p| p.retain(|&t| s != t));
        if s.starts_with("departure") { departure *= input.tickets[0][i] }
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
    let mut i = s.split("\n\n");
    let field_ranges = i.next().unwrap().lines().map(parse_field_range).collect();
    let tickets      = i.next().unwrap()
                        .lines()
                        .skip(1)
                        .chain(i.next().unwrap().lines().skip(1))
                        .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
                        .collect();

    TicketInfo { field_ranges, tickets }
}

fn parse_field_range(s : &str) -> (&str, Vec<RangeInclusive<u64>>)
{
    let mut i = s.split(": ");
    let field = i.next().unwrap();
    let range = i.next().unwrap()
                 .split(" or ")
                 .map(|r| { let mut j = r.split('-'); j.next().unwrap().parse().unwrap() ..= j.next().unwrap().parse().unwrap() })
                 .collect();

    (field, range)
}
