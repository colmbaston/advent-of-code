use std::collections::VecDeque;

fn main()
{
    let input      = include_str!("../input.txt").trim_end().parse::<usize>().unwrap();
    let mut buffer = std::iter::once(0).collect::<VecDeque<u32>>();
    for k in 1 ..= 50_000_000
    {
        buffer.rotate_left((input+1) % buffer.len());
        buffer.push_front(k);

        if k == 2017 { println!("{}", buffer[1]) }
    }
    let zero = buffer.iter().position(|&k| k == 0).unwrap();
    println!("{}", buffer[(zero + 1) % buffer.len()]);
}
