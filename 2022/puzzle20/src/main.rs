use std::collections::VecDeque;

fn main()
{
    let mut file   = include_str!("../input.txt").lines().filter_map(|l| l.parse().ok()).collect::<Vec<i64>>();
    let mut buffer = VecDeque::new();

    for two in [false, true]
    {
        if two { for val in file.iter_mut() { *val *= 811_589_153 }}

        buffer.clear();
        buffer.extend(0 .. file.len());

        for _ in 0 .. if two { 10 } else { 1 }
        {
            mix(&file, &mut buffer);
        }

        let zero = buffer.iter().position(|&i| file[i] == 0).unwrap_or(0);
        println!("{}", (1 ..= 3).map(|i| file[buffer[(zero + i*1000) % file.len()]]).sum::<i64>());
    }
}

fn rotate<T>(buffer : &mut VecDeque<T>, rot : i64)
{
    let fun = if rot < 0 { VecDeque::rotate_right } else { VecDeque::rotate_left };
    fun(buffer, rot.unsigned_abs() as usize % buffer.len());
}

fn mix(file : &[i64], buffer : &mut VecDeque<usize>)
{
    for i in 0 .. file.len()
    {
        rotate(buffer, buffer.iter().position(|&j| j == i).unwrap_or(0) as i64);
        buffer.pop_front();
        rotate(buffer, file[i]);
        buffer.push_front(i);
    }
}
