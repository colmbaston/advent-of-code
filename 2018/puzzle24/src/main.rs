use std::cmp::Reverse;
use std::collections::{ HashSet, HashMap, hash_map::Entry };

fn main()
{
    let groups = parse(include_str!("../input.txt"));

    // part 1: what would the outcome of the combat be?
    println!("{:?}", combat(groups.clone()).1);

    // part 2: find the minimum boost such that the
    // immune system wins, then print that outcome
    for boost in 1 ..
    {
        let mut groups = groups.clone();
        for (&(t, _), g) in groups.iter_mut()
        {
            if t == Team::ImmuneSystem
            {
                g.attack_damage += boost
            }
        }

        if let (Some(Team::ImmuneSystem), remaining) = combat(groups)
        {
            println!("{}", remaining);
            break
        }
    }

}

fn combat(mut groups : HashMap<(Team, usize), Group>) -> (Option<Team>, u32)
{
    // reuse the memory from previous iterations
    let mut action_order = Vec::new();
    let mut targets      = HashMap::new();
    let mut targeted     = HashSet::new();
    let mut units_before = Group::sum_units(groups.values());

    loop
    {
        let immune_remaining = groups.keys().filter(|(t, _)| t == &Team::ImmuneSystem).count();

        if immune_remaining == 0
        {
            break (Some(Team::Infection), units_before)
        }
        else if immune_remaining == groups.len()
        {
            break (Some(Team::ImmuneSystem), units_before)
        }

        targets.clear();
        targeted.clear();

        // groups select targets in an order based on their effective power and initiative
        action_order.extend(groups.keys().copied());
        action_order.sort_by_key(|i| { let g = groups.get(i).unwrap(); Reverse((g.effective_power(), g.initiative)) });

        // target selection phase:
        for (current_i, current_g) in action_order.drain(..).map(|i| (i, groups.get(&i).unwrap()))
        {
            let best_target = groups.iter().filter_map(|(&other_i, other_g)|
            {
                if current_i.0 != other_i.0 && !targeted.contains(&other_i)
                {
                    let damage = current_g.damage(other_g);
                    if damage > 0
                    {
                        Some((damage,
                              other_g.effective_power(),
                              other_g.initiative,
                              other_i))
                    }
                    else
                    {
                        None
                    }
                }
                else
                {
                    None
                }
            })
            .max();

            if let Some((_, _, _, target_i)) = best_target
            {
                targets.insert(current_i, target_i);
                targeted.insert(target_i);
            }
        }

        // groups attack in an order based only on their initiative
        action_order.extend(groups.keys().copied());
        action_order.sort_by_key(|i| Reverse(groups.get(i).unwrap().initiative));

        // attacking phase
        for current_i in action_order.drain(..)
        {
            if let Some(current_g) = groups.remove(&current_i)
            {
                if let Some(Entry::Occupied(mut e)) = targets.get(&current_i).map(|&i| groups.entry(i))
                {
                    let target_g = e.get_mut();
                    let losses = current_g.damage(target_g) / target_g.hit_points;
                    if target_g.units <= losses { e.remove(); } else { target_g.units -= losses }
                }

                groups.insert(current_i, current_g);
            }
        }

        // if the total unit count has not changed, the
        // combat does not end decisively for either team
        let units_after = Group::sum_units(groups.values());
        if units_after == units_before
        {
            break (None, units_before)
        }

        units_before = units_after;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Team
{
    ImmuneSystem,
    Infection
}

#[derive(Debug, Clone)]
struct Group<'a>
{
    units:         u32,
    hit_points:    u32,
    immunities:    Vec<&'a str>,
    weaknesses:    Vec<&'a str>,
    attack_damage: u32,
    attack_type:   &'a str,
    initiative:    u32
}

fn parse(s : &str) -> HashMap<(Team, usize), Group>
{
    let mut groups = HashMap::new();
    for (s, t) in s.split("\n\n").zip(vec![Team::ImmuneSystem, Team::Infection].into_iter())
    {
        groups.extend(s.lines().skip(1).zip(1..).map(|(s, i)| ((t, i), Group::parse(s))))
    }
    groups
}

impl<'a> Group<'a>
{
    fn parse(s : &str) -> Group
    {
        fn span(s : &str, pred : impl Fn(char) -> bool) -> (&str, &str)
        {
            s.split_at(s.find(|c| !pred(c)).unwrap_or(s.len()))
        }

        fn parse_digits(s : &str) -> (u32, &str)
        {
            let (digits, s) = span(s, |c| c.is_ascii_digit());
            (digits.parse().unwrap(), s)
        }

        let (units,      s) = parse_digits(s);
        let (hit_points, s) = parse_digits(&s[17..]);

        let mut immunities = Vec::new();
        let mut weaknesses = Vec::new();

        let s = if s.as_bytes()[12] == b'('
        {
            let (before, after) = span(&s[13..], |c| c != ')');

            for section in before.split("; ")
            {
                if section.as_bytes()[0] == b'i'
                {
                    immunities.extend(section[10..].split(", "));
                }
                else
                {
                    weaknesses.extend(section[8..].split(", "));
                }
            }

            &after[27..]
        }
        else
        {
            &s[37..]
        };

        let (attack_damage, s) = parse_digits(&s[0..]);
        let (attack_type,   s) = span(&s[1..], |c| !c.is_ascii_whitespace());
        let (initiative,    _) = parse_digits(&s[22..]);

        Group
        {
            units,
            hit_points,
            immunities,
            weaknesses,
            attack_damage,
            attack_type,
            initiative
        }
    }

    fn effective_power(&self) -> u32
    {
        self.units * self.attack_damage
    }

    fn damage(&self, other : &Group) -> u32
    {
        if other.immunities.contains(&self.attack_type)
        {
            0
        }
        else
        {
            let mut damage = self.effective_power();
            if other.weaknesses.contains(&self.attack_type) { damage *= 2 }
            damage
        }
    }

    fn sum_units(groups : impl Iterator<Item = &'a Group<'a>>) -> u32
    {
        groups.map(|g| g.units).sum()
    }
}
