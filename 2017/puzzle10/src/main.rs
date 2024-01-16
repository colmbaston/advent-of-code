#![feature(array_chunks, iter_next_chunk)]
use std::collections::VecDeque;

fn main()
{
    let input = include_str!("../input.txt").trim_end();

    println!("{}", rounds(1, &input.split(',')
                                   .map(|k| k.parse().unwrap())
                                   .collect::<Vec<u8>>()).into_iter().map(|k| k as u32)
                                                         .take(2).product::<u32>());

    knot_hash(input.as_bytes()).iter().for_each(|b| print!("{b:x}"));
    println!();
}

fn knot_hash(data : &[u8]) -> [u8 ; 16]
{
    let mut data = data.to_vec();
    data.extend([17, 31, 73, 47, 23]);
    rounds(64, &data).make_contiguous()
                     .array_chunks()
                     .map(|chunk : &[u8 ; 16]| chunk.iter().fold(0, |a, b| a ^ b))
                     .next_chunk().unwrap()
}

fn rounds(rounds : usize, data : &[u8]) -> VecDeque<u8>
{
    let mut knot    = (0 ..= 255).collect::<VecDeque<u8>>();
    let mut skip    = 0;
    let mut net_rot = 0;

    for _ in 0 .. rounds
    {
        for &len in data.iter()
        {
            knot.make_contiguous()[.. len as usize].reverse();
            let rot = len as usize + skip;
            knot.rotate_left(rot % 256);
            net_rot += rot;
            skip    += 1;
        }
    }

    knot.rotate_right(net_rot % 256);
    knot
}
