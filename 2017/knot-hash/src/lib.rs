#![feature(iter_array_chunks, iter_next_chunk)]

use std::collections::VecDeque;

pub fn hash(data : &[u8]) -> [u8 ; 16]
{
    let mut data = data.to_vec();
    data.extend([17, 31, 73, 47, 23]);
    rounds(64, &data).into_iter()
                     .array_chunks()
                     .map(|chunk : [u8 ; 16]| chunk.iter().fold(0, |a, b| a ^ b))
                     .next_chunk().unwrap()
}

pub fn rounds(rounds : usize, data : &[u8]) -> VecDeque<u8>
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
