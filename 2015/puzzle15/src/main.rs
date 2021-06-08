fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_row).collect::<Vec<Vec<i32>>>();

    let (a, b) = splits(100, input.len()).fold((0, 0), |(a, b), s|
    {
        let     ps  = properties(&s, &input);
        let (c, ps) = ps.split_last().unwrap();
        let score   = ps.iter().product();

        (a.max(score), if *c == 500 { b.max(score) } else { b })
    });

    println!("{}", a);
    println!("{}", b);
}

fn parse_row(s : &str) -> Vec<i32>
{
    s.split(|c| c == ' ' || c == ':' || c == ',')
     .filter_map(|s| s.parse().ok())
     .collect()
}

fn splits(m : u32, n : usize) -> Box<dyn Iterator<Item = Vec<u32>>>
{
    match (m, n)
    {
        (0, _) => Box::new(std::iter::once(vec![0 ; n])),
        (_, 0) => Box::new(std::iter::empty()),
        _      => Box::new((0 ..= m).flat_map(move |k| splits(m-k, n-1).map(move |mut v| { v.push(k); v })))
    }
}

fn properties(split : &[u32], ingredients : &[Vec<i32>]) -> Vec<i32>
{
    (0 .. ingredients[0].len()).map(|p| split.iter()
                                             .zip(ingredients.iter())
                                             .map(|(s, i)| *s as i32 * i[p])
                                             .sum::<i32>()
                                             .max(0))
                               .collect()
}
