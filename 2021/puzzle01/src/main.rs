fn main()
{
    let input = include_str!("../input.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut prev  = None;
    let mut count = 0;
    for &d in input.iter()
    {
        if let Some(e) = prev
        {
            if d > e { count += 1 }
        }
        prev = Some(d);
    }
    println!("{}", count);

    prev  = None;
    count = 0;
    for ds in input.windows(3)
    {
        let d = ds.iter().sum();
        if let Some(e) = prev
        {
            if d > e { count += 1 }
        }
        prev = Some(d);
    }
    println!("{}", count);
}
