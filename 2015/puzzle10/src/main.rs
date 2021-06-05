fn main()
{
    let mut input = include_str!("../input.txt").trim_end().bytes().collect::<Vec<u8>>();
    let mut next  = Vec::new();

    for i in 1 ..= 50
    {
        next.clear();
        let mut bs = input.iter().peekable();

        while let Some(c) = bs.next()
        {
            let mut n = 1;
            while bs.peek() == Some(&c)
            {
                bs.next();
                n += 1;
            }
            next.extend(format!("{}{}", n, c - b'0').bytes());
        }

        std::mem::swap(&mut input, &mut next);

        if let 40 | 50 = i
        {
            println!("{}", input.len());
        }
    }
}
