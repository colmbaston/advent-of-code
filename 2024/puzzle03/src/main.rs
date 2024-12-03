fn main()
{
    let memory = include_str!("../input.txt");
    println!("{}", scan_mul(memory).sum::<u32>());
    println!("{}", scan_cond(memory).sum::<u32>());
}

fn scan_mul(s : &str) -> impl Iterator<Item = u32> + '_
{
    s.split("mul(").skip(1).filter_map(|a|
    {
        let (x, b) = a.split_once(',')?;
        let (y, _) = b.split_once(')')?;
        Some(x.parse::<u32>().ok()? * y.parse::<u32>().ok()?)
    })
}

fn scan_cond(s : &str) -> impl Iterator<Item = u32> + '_
{
    s.split("do()")
     .flat_map(|a| scan_mul(a.split_once("don't()")
                             .map(|(b, _)| b)
                             .unwrap_or(a)))
}
