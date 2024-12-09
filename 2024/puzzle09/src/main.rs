fn main()
{
    let blocks = include_str!("../input.txt").trim_end().bytes().zip(0 ..)
                                             .zip([false, true].into_iter().cycle())
                                             .flat_map(|((b, id), free)| std::iter::repeat((!free).then_some(id/2)).take((b - b'0') as usize))
                                             .collect::<Vec<Option<u32>>>();

    let mut defrag = blocks.clone();
    for free in blocks.iter().enumerate().filter_map(|(i, b)| b.is_none().then_some(i))
    {
        while let Some(None) = defrag.last() { defrag.pop(); }
        let len = defrag.len();
        if free < len { defrag.swap(free, len-1) } else { break }
    }
    println!("{}", defrag.into_iter().zip(0 ..).map(|(b, i)| b.unwrap() as u64 * i).sum::<u64>());

    let mut files = Vec::new();
    let mut free  = Vec::new();
    for (index, chunk) in blocks.chunk_by(|a, b| a == b).scan(0, |i, c| { let j = *i; *i += c.len(); Some((j, c)) })
    {
        match chunk[0]
        {
            None    => free.push((index, chunk.len())),
            Some(_) => files.push((index, chunk.len()))
        }
    }
    println!("{}", files.into_iter().enumerate().rev().map(|(id, (mut file_index, file_size))|
    {
        if let Some((free_index, free_size)) = free.iter_mut().take_while(|(i, _)| *i < file_index).find(|(_, s)| *s >= file_size)
        {
            file_index  = *free_index;
            *free_index += file_size;
            *free_size  -= file_size;
        }

        let size = file_size as u64;
        id as u64 * (file_index as u64 * size + size * (size-1) / 2)
    })
    .sum::<u64>());
}
