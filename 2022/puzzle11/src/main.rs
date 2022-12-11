use std::collections::BinaryHeap;

mod monkey;
use monkey::Monkey;

fn main()
{
    let mut monkeys  = include_str!("../input.txt").split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    let mut activity = vec![0 ; monkeys.len()];

    for _ in 0 .. 20
    {
        for (monkey_ix, inspections) in activity.iter_mut().enumerate()
        {
            if let Some(mut monkey) = monkeys.get_mut(monkey_ix).and_then(Option::take)
            {
                while let Some(mut worry) = monkey.items.pop_front()
                {
                    worry = monkey.op(worry) / 3;
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

    let mut heap = activity.iter().copied().collect::<BinaryHeap<usize>>();
    println!("{}", std::iter::from_fn(|| heap.pop()).take(2).product::<usize>());
}
