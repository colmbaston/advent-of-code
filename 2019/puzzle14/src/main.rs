use parsing::*;
use std::collections::HashMap;

fn main()
{
    let reactions    = include_str!("../input.txt").lines().map(|s| parse_reaction(s).unwrap().1).collect();
    let mut leftover = HashMap::new();

    println!("{}", ore_required(1, "FUEL", &reactions, &mut leftover));
    leftover.clear();

    let mut upper = 1;
    while ore_required(upper, "FUEL", &reactions, &mut leftover) <= 1_000_000_000_000
    {
        leftover.clear();
        upper *= 2;
    }

    let mut lower = upper / 2;
    while upper - lower > 1
    {
        let middle = (lower + upper) / 2;
        if ore_required(middle, "FUEL", &reactions, &mut leftover) > 1_000_000_000_000
        {
            upper = middle;
        }
        else
        {
            lower = middle;
        }
        leftover.clear();
    }

    println!("{}", lower);
}

fn parse_reaction(s : &str) -> IResult<&str, (&str, (u64, Vec<(u64, &str)>))>
{
    let chem = |s| separated_pair(natural, char(' '), alpha1)(s);
    let (s, (inputs, (q, output))) = separated_pair(separated_list(tag(", "), chem), tag(" => "), chem)(s)?;
    Ok((s, (output, (q, inputs))))
}

fn ore_required<'a>(mut q_required : u64, chem : &'a str, reactions : &HashMap<&'a str, (u64, Vec<(u64, &'a str)>)>, leftover : &mut HashMap<&'a str, u64>) -> u64
{
    if let Some(q_left) = leftover.get_mut(chem)
    {
        if *q_left >= q_required
        {
            *q_left -= q_required;
            return 0
        }
        else
        {
            q_required -= *q_left;
            *q_left = 0;
        }
    }

    if chem == "ORE"
    {
        return q_required
    }

    match reactions.get(chem)
    {
        None => panic!("chemical {} not produced by any reaction!", chem),
        Some((q_output, inputs)) =>
        {
            let runs     = (q_required + q_output - 1) / q_output;
            let produced = q_output * runs;
            let left     = produced - q_required;
            leftover.entry(chem).and_modify(|q_left| *q_left += left).or_insert(left);
            inputs.iter().map(|&(q_input, chem_input)| ore_required(q_input * runs, chem_input, reactions, leftover)).sum()
        }
    }
}
