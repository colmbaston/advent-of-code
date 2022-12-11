use std::collections::BinaryHeap;

mod monkey;
use monkey::{ Monkey, WorryLevel };

fn main()
{
    let mut monkeys_one = include_str!("../input.txt").split("\n\n").map(Monkey::parse).collect::<Vec<Option<Monkey>>>();
    let mut monkeys_two = monkeys_one.clone();
    let mut activity    = vec![0 ; monkeys_one.len()];

    for _ in 0 .. 20 { round(&mut monkeys_one, &mut activity, |worry| worry / 3) }
    let mut heap = activity.iter().copied().collect::<BinaryHeap<u64>>();
    println!("{}", std::iter::from_fn(|| heap.pop()).take(2).product::<u64>());

    drop(monkeys_one);
    for inspections in activity.iter_mut() { *inspections = 0 }
    let prod = Monkey::product(monkeys_two.iter().filter_map(|m| m.as_ref()));

    for _ in 0 .. 10000 { round(&mut monkeys_two, &mut activity, move |worry| worry % prod) }
    heap.clear();
    heap.extend(activity.iter().copied());
    println!("{}", std::iter::from_fn(|| heap.pop()).take(2).product::<u64>());
}

fn round(monkeys : &mut [Option<Monkey>], activity : &mut [u64], calm : impl Fn(WorryLevel) -> WorryLevel)
{
    for (monkey_ix, inspections) in activity.iter_mut().enumerate()
    {
        if let Some(mut monkey) = monkeys.get_mut(monkey_ix).and_then(Option::take)
        {
            while let Some(mut worry) = monkey.items.pop_front()
            {
                worry = calm(monkey.op(worry));
                if let Some(Some(dest_monkey)) = monkeys.get_mut(monkey.throw_to(worry))
                {
                    dest_monkey.items.push_back(worry);
                }
                *inspections += 1;
            }
            if let Some(entry) = monkeys.get_mut(monkey_ix) { *entry = Some(monkey) };
        }
    }
}
