use std::collections::VecDeque;

fn main()
{
    let file       = include_str!("../input.txt").lines().filter_map(|l| l.parse().ok()).collect::<Vec<i64>>();
    let mut buffer = VecDeque::new();

    for (key, rounds) in [(1, 1), (811_589_153, 10)]
    {
        buffer.clear();
        buffer.extend(0 .. file.len());

        for _ in 0 .. rounds { mix(&file, key, &mut buffer) }

        let zero = buffer.iter().position(|&i| file[i] == 0).unwrap_or(0);
        println!("{}", (1 ..= 3).map(|i| key * file[buffer[(zero + i*1000) % file.len()]]).sum::<i64>());
    }
}

fn mix(file : &[i64], key : i64, buffer : &mut VecDeque<usize>)
{
    for (i, val) in file.iter().map(|val| key * val).enumerate()
    {
        buffer.rotate_left(buffer.iter().position(|&j| j == i).unwrap_or(0));
        buffer.pop_front();

        let rot = (val.unsigned_abs() % buffer.len() as u64) as usize;
        if val.is_positive() { buffer.rotate_left(rot) } else { buffer.rotate_right(rot) }
        buffer.push_front(i);
    }
}
