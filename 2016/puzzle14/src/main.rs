#![feature(array_windows)]
use md5::{ Md5, Digest };
use std::collections::VecDeque;

fn main()
{
    let input      = include_str!("../input.txt").trim_end();
    let mut hashes = VecDeque::new();

    for stretch in [0, 2016]
    {
        hashes.clear();
        let mut count = 0;

        for i in 0 ..
        {
            let mut hasher = Md5::new();
            hasher.update(input);
            hasher.update(format!("{i}"));
            let mut hash = format!("{:x}", hasher.finalize());

            for _ in 0 .. stretch
            {
                hasher = Md5::new();
                hasher.update(hash);
                hash = format!("{:x}", hasher.finalize());
            }

            if let Some(hash) = saturate(&mut hashes, hash, 1000)
            && let Some(k) = hash.as_bytes().array_windows::<3>().find_map(|[k, rest@..]| rest.iter().all(|l| k == l).then_some(k))
            && hashes.iter().any(|h| h.as_bytes().array_windows::<5>().any(|a| a.iter().all(|l| k == l)))
            {
                count += 1;
                if count == 64
                {
                    println!("{}", i - 1000);
                    break
                }
            }
        }
    }
}

fn saturate<T>(queue : &mut VecDeque<T>, elem : T, target : usize) -> Option<T>
{
    queue.push_back(elem);
    (queue.len() > target).then(|| queue.pop_front()).flatten()
}
