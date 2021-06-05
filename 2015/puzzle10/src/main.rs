fn main()
{
    let mut input = String::from(include_str!("../input.txt").trim_end());
    let mut next  = String::new();

    for i in 1 ..= 50
    {
        next.clear();
        let mut cs = input.chars();
        let mut c  = cs.next().unwrap();
        let mut n  = 1;

        for d in cs
        {
            if c == d
            {
                n += 1
            }
            else
            {
                next.extend(format!("{}{}", n, c).chars());
                c = d;
                n = 1;
            }
        }

        next.extend(format!("{}{}", n, c).chars());
        std::mem::swap(&mut input, &mut next);

        if let 40 | 50 = i
        {
            println!("{}", input.len());
        }
    }
}
