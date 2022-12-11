mod monkey;
use monkey::{ Monkey, WorryLevel };

fn main()
{
    let mut monkeys_one = include_str!("../input.txt").split("\n\n").filter_map(Monkey::parse).collect::<Vec<Monkey>>();
    let mut monkeys_two = monkeys_one.clone();
    let mut activity    = vec![0 ; monkeys_one.len()];

    rounds(20, &mut monkeys_one, &mut activity, |worry| worry / 3);
    activity.sort_unstable();
    println!("{}", activity.iter().rev().take(2).product::<u64>());

    drop(monkeys_one);
    for inspections in activity.iter_mut() { *inspections = 0 }
    let prod = Monkey::product(monkeys_two.iter());

    rounds(10000, &mut monkeys_two, &mut activity, move |worry| worry % prod);
    activity.sort_unstable();
    println!("{}", activity.into_iter().rev().take(2).product::<u64>());
}

fn rounds(count : u32, monkeys : &mut [Monkey], activity : &mut [u64], calm : impl Fn(WorryLevel) -> WorryLevel)
{
    let mut throws = Vec::new();
    for _ in 0 .. count
    {
        for (monkey_ix, inspections) in activity.iter_mut().enumerate()
        {
            let monkey = &mut monkeys[monkey_ix];
            while let Some(mut worry) = monkey.items.pop_front()
            {
                worry = calm(monkey.op(worry));
                throws.push((worry, monkey.throw_to(worry)));
                *inspections += 1;
            }
            for (worry, dest_ix) in throws.drain(..)
            {
                monkeys[dest_ix].items.push_back(worry)
            }
        }
    }
}
