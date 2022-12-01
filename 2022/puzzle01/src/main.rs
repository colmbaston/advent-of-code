use std::collections::BinaryHeap;

fn main()
{
    let mut heap = include_str!("../input.txt").split("\n\n")
                                               .map(|group| group.lines().map(|l| l.parse::<u32>().unwrap_or(0)).sum())
                                               .collect::<BinaryHeap<u32>>();

    println!("{}", heap.peek().unwrap());
    println!("{}", std::iter::from_fn(|| heap.pop()).take(3).sum::<u32>());
}
